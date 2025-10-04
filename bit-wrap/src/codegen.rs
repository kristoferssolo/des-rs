use crate::ast::Struct;
use quote::quote;
use unsynn::*;

#[ignore = "too_many_lines"]
pub fn generate_impl(info: &Struct) -> TokenStream {
    let name = &info.name;
    let inner = &info.body;

    let ops = generate_bitwise_ops(info);
    let fmt = generate_bitwise_fmt(info);
    let wrapper = generate_bitwise_wrapper(info);

    quote! {
        #ops
        #fmt
        #wrapper

        impl std::convert::From<#name> for #inner {
            fn from(value: #name) -> Self {
                value.0
            }
        }

        impl AsRef<#inner> for #name {
            fn as_ref(&self) -> &#inner {
                &self.0
            }
        }

        impl std::ops::Deref for #name {
            type Target = #inner;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    }
}

fn generate_bitwise_fmt(info: &Struct) -> TokenStream {
    let name = &info.name;

    quote! {
        impl std::fmt::LowerHex for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::LowerHex::fmt(&self.0, f)
            }
        }

        impl std::fmt::UpperHex for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::UpperHex::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for Subkey {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Subkey(0x{:012X})", self.0)
            }
        }

        impl std::fmt::Display for Subkey {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "0x{:012X}", self.0)
            }
        }
    }
}

fn generate_bitwise_ops(info: &Struct) -> TokenStream {
    let name = &info.name;
    let inner = &info.body;
    let error_type = &info.error_type;

    let bit_width = u8::try_from(info.bit_width).expect("8-bit value");

    let hex_width = usize::from(bit_width.div_ceil(4));
    let bin_width = usize::from(bit_width);
    let max_value = (1u64 << (bit_width)) - 1;

    quote! {
        impl #name {
            /// The bit width of this type
            pub const BIT_WIDTH: u8 = #bit_width;

            /// Minimum value for this bit width
            pub const MIN: #inner = 0;

            /// Maximum value for this bit width
            pub const MAX: #inner = #max_value;

            /// Create a new [`Self`] from a key value
            #[inline]
            #[macro_use]
            pub fn new(key: #inner) -> Result<Self, #error_type> {
                key.try_into()
            }

            /// Convert to hex string (formatted for the bit width)
            #[macro_use]
            pub fn to_hex(self) -> String {
                let value = self.0 & Self::MAX;
                format!("{:0width$X}", value, width = #hex_width)
            }

            /// Convert to binary string (full bit width with leading zeros)
            #[macro_use]
            pub fn to_bin(self) -> String {
                let value = self.0 & Self::MAX;
                format!("{:0width$b}", value, width = #bin_width)
            }

            /// Check if all bits are set
            #[inline]
            #[macro_use]
            pub const fn all(self) -> bool {
                self.0 == Self::MAX
            }

            /// Check if any bit is set
            #[inline]
            #[macro_use]
            pub const fn any(self) -> bool {
                self.0 != 0
            }

            /// Count the number of set bits
            #[inline]
            #[macro_use]
            pub const fn count_ones(self) -> u32 {
                self.0.count_ones()
            }

            /// Count the number of zero bits within the constrained bit width
            #[macro_use]
            pub const fn count_zeros(self) -> u32 {
                let value = self.0 as #inner & Self::MAX;
                let ones_count = self.count_ones();
                (Self::BIT_WIDTH as u32) - ones_count
            }

            /// Reverse the bit pattern within the constrained bit width
            #[macro_use]
            pub const fn reverse_bits(self) -> Self {
                let value = self.0 as #inner & Self::MAX;
                let reversed = value.reverse_bits();
                // Mask and shift to keep only the relevant bits
                let result = ((reversed >> (64 - Self::BIT_WIDTH)) & Self::MAX).reverse_bits()
                            >> (64 - Self::BIT_WIDTH);
                Self((result & Self::MAX) as #inner)
            }


            /// Rotate the bits left by `n` positions within the bit width
            #[macro_use]
            pub const fn rotate_left(self, n: u8) -> Self {
                let n = n % Self::BIT_WIDTH;
                if n == 0 {
                    return self;
                }

                let masked =  self.0 & Self::MAX;
                let rotated = ((masked << n) | (masked >> Self::BIT_WIDTH.saturating_sub(n))) & Self::MAX;

                #name(rotated)
            }

            /// Rotate the bits right by `n` positions within the bit width
            #[macro_use]
            pub const fn rotate_right(self, n: u8) -> Self {
                let n = n % Self::BIT_WIDTH;
                if n == 0 {
                    return self;
                }

                let masked =  self.0 & Self::MAX;
                let rotated = ((masked >> n) | (masked << Self::BIT_WIDTH.saturating_sub(n))) & Self::MAX;

                #name(rotated)
            }

            /// Find the number of leading zero bits within the bit width
            #[macro_use]
            pub fn leading_zeros(self) -> u32 {
                let value = self.0 as #inner & Self::MAX;
                let bit_width = Self::BIT_WIDTH as u32;
                if value == 0 {
                    return bit_width;
                }
                let full_leading_zeros = value.leading_zeros();
                let adjustment = 64u32.saturating_sub(bit_width);
                full_leading_zeros.saturating_sub(adjustment).min(bit_width)
            }

            /// Find the number of trailing zero bits within the bit width
            #[macro_use]
            pub fn trailing_zeros(self) -> u32 {
                let value = self.0 as #inner & Self::MAX;
                let reversed = self.reverse_bits();
                reversed.leading_zeros()
            }

            /// Find the number of leading one bits from the MSB within the bit width
            #[macro_use]
            pub fn leading_ones(self) -> u32 {
                let value = self.0 as #inner & Self::MAX;
                let not_value = !value & Self::MAX;
                let bit_width = Self::BIT_WIDTH as u32;

                if not_value == 0 {
                    return bit_width;
                }

                bit_width - not_value.trailing_zeros().min(bit_width)
            }

            /// Find the number of trailing one bits from the LSB within the bit width
            #[macro_use]
            pub fn trailing_ones(self) -> u32 {
                let value = self.0 as #inner & Self::MAX;
                let reversed = self.reverse_bits();
                reversed.leading_ones()
            }

            /// Check if the value is zero within the bit width
            #[macro_use]
            pub const fn is_zero(self) -> bool {
                (self.0 & Self::MAX)  == 0
            }

            /// Set a specific bit to 1 (clamped to bit width)
            #[macro_use]
            pub const fn bit_is_set(self, bit: u8) -> bool {
                let clamped = self.clamp_bit_positions(bit);
                let mask = 1<< clamped;
                (self.0 & mask) == mask
            }

            /// Set a specific bit to 1 (clamped to bit width)
            pub const fn set_bit(&mut self, bit: u8) {
                let clamped = self.clamp_bit_positions(bit);
                self.0 |= 1 << clamped;
            }

            /// Clear a specific bit to 0 (clamped to bit width)
            pub const fn clear_bit(&mut self, bit: u8) {
                let clamped = self.clamp_bit_positions(bit);
                let mask = !(1 << clamped);
                self.0 &= (mask & Self::MAX);
            }

            /// Toggle a specific bit (clamped to bit width)
            pub const fn toggle_bit(&mut self, bit: u8) {
                let clamped = self.clamp_bit_positions(bit);
                self.0 ^= 1 << clamped;
            }

            /// Check if bits in the given mask are set (within bit width)
            #[macro_use]
            pub const fn contains_mask(self, mask: Self) -> bool {
                let self_value = self.0 & Self::MAX;
                let mask_value = mask.0 & Self::MAX;
                (self_value & mask_value) == mask.0
            }

            /// Create from a hex string with bit width validation
            pub fn from_hex(hex: &str) -> Result<Self, #error_type> {
                let value = #inner::from_str_radix(hex, 16)?;

                let masked = value & Self::MAX;
                if value != masked {
                    return Err(#error_type::ExceedsBitLimit {
                        value,
                        bit_width: Self::BIT_WIDTH,
                        masked,
                        width: #hex_width
                    })
                }

                Ok(Self(value))
            }

            /// Extract a bitfield from the constrained range
            #[macro_use]
            pub const fn bitfield(self, low: u8, high: u8) -> u64 {
                let low = self.clamp_bit_positions(low);
                let high = self.clamp_bit_positions(high);

                let start= if low <= high { low }else{ high };
                let end = if low <= high { high }else{ low };

                let width = high.wrapping_sub(low).saturating_sub(1);

                if width == 0 {
                    return 0;
                }

                let value = self.0 & Self::MAX;
                (value >> low) & ((1 << width) - 1)
            }

            /// Set bits from a constrained range [low, high]
            pub const fn set_bitfield(&mut self, value: u64, low: u32, high: u32) {
                let mask = ((1 << (high - low + 1)) - 1) << low;
                self.0 = (self.0 & !mask) | ((value << low) & mask);
            }

            #[macro_use]
            const fn clamp_bit_positions(self, bit: u8) -> u8 {
                if bit >= Self::BIT_WIDTH {
                    return Self::BIT_WIDTH - 1;
                }
                bit
            }
        }
    }
}

fn generate_bitwise_wrapper(_info: &Struct) -> TokenStream {
    quote! {
        impl std::ops::Not for Subkey {
            type Output = Self;
            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }

        impl std::ops::BitAnd for Subkey {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl std::ops::BitAnd<&Self> for Subkey {
            type Output = Self;
            fn bitand(self, rhs: &Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl std::ops::BitAndAssign for Subkey {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl std::ops::BitAndAssign<&Self> for Subkey {
            fn bitand_assign(&mut self, rhs: &Self) {
                self.0 &= rhs.0;
            }
        }

        impl std::ops::BitOr for Subkey {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl std::ops::BitOr<&Self> for Subkey {
            type Output = Self;
            fn bitor(self, rhs: &Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl std::ops::BitOrAssign for Subkey {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl std::ops::BitOrAssign<&Self> for Subkey {
            fn bitor_assign(&mut self, rhs: &Self) {
                self.0 |= rhs.0;
            }
        }

        impl std::ops::BitXor for Subkey {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }

        impl std::ops::BitXor<&Self> for Subkey {
            type Output = Self;
            fn bitxor(self, rhs: &Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }

        impl std::ops::BitXorAssign for Subkey {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }

        impl std::ops::BitXorAssign<&Self> for Subkey {
            fn bitxor_assign(&mut self, rhs: &Self) {
                self.0 ^= rhs.0;
            }
        }

        impl std::ops::Shl<u32> for Subkey {
            type Output = Self;
            fn shl(self, rhs: u32) -> Self {
                Self(self.0 << rhs)
            }
        }

        impl std::ops::ShlAssign<u32> for Subkey {
            fn shl_assign(&mut self, rhs: u32) {
                self.0 <<= rhs;
            }
        }

        impl std::ops::Shr<u32> for Subkey {
            type Output = Self;
            fn shr(self, rhs: u32) -> Self::Output {
                Self(self.0 >> rhs)
            }
        }

        impl std::ops::ShrAssign<u32> for Subkey {
            fn shr_assign(&mut self, rhs: u32) {
                self.0 >>= rhs;
            }
        }
    }
}
