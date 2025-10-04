use crate::ast::Struct;
use quote::quote;
use unsynn::{Ident, TokenStream};

#[allow(clippy::too_many_lines)]
pub fn generate_impl(info: &Struct) -> TokenStream {
    let name = &info.name;
    let inner = &info.body;

    let error = generate_error(info);
    let conversions = generate_conversions(info);
    let ops = generate_bitwise_ops(info);
    let fmt = generate_bitwise_fmt(info);
    let wrapper = generate_bitwise_wrapper(info);

    quote! {
        #error
        #conversions
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

fn generate_conversions(info: &Struct) -> TokenStream {
    let name = &info.name;
    let inner = &info.body;
    let error_type = &info.error_type;
    let bit_width = info.bit_width;

    // Generate From implementations for smaller integer types (guaranteed safe)
    let impl_from = |target_width: u8, srouce_type: &TokenStream| {
        generate_from_impl(bit_width, target_width, srouce_type, name, inner)
    };

    let from_u8 = impl_from(8, &quote! { u8 });
    let from_u16 = impl_from(15, &quote! { u16 });
    let from_u32 = impl_from(32, &quote! { u32 });
    let from_u64 = impl_from(64, &quote! { u64 });

    let impl_try_from = |target_width: u8, srouce_type: &TokenStream| {
        generate_try_from_impl(
            bit_width,
            target_width,
            srouce_type,
            name,
            inner,
            error_type,
        )
    };
    let try_from_u8 = impl_try_from(8, &quote! { u8 });
    let try_from_u16 = impl_try_from(16, &quote! { u16 });
    let try_from_u32 = impl_try_from(32, &quote! { u32 });
    let try_from_u64 = impl_try_from(64, &quote! { u64 });

    quote! {
        #from_u8
        #from_u16
        #from_u32
        #from_u64
        #try_from_u8
        #try_from_u16
        #try_from_u32
        #try_from_u64
    }
}

fn generate_error(info: &Struct) -> TokenStream {
    let name = &info.name;
    let error_type = &info.error_type;
    let inner = &info.body;

    quote! {
        #[derive(Debug, thiserror::Error)]
        pub enum #error_type {
            #[error("#name must be a {width}-bit number (0 to {max}), got {value}")]
            ValueOutOfRange {
                value: #inner,
                max: #inner,
                width: u8
            },

            #[error("Failed to parse subkey value: {0}")]
            ParseError(#[from] std::num::ParseIntError),

            #[error("Unknown error: {0}")]
            Unknown(String),
        }

        impl #error_type {
            pub fn value_out_of_range(value: #inner) -> Self {
                Self::ValueOutOfRange {
                    value,
                    max: #name::MAX,
                    width: #name::BIT_WIDTH,
                }
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

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "#name(0x{:012X})", self.0)
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "0x{:012X}", self.0)
            }
        }
    }
}

#[allow(clippy::similar_names)]
#[allow(clippy::too_many_lines)]
fn generate_bitwise_ops(info: &Struct) -> TokenStream {
    let name = &info.name;
    let inner = &info.body;
    let error_type = &info.error_type;

    let bit_width = info.bit_width;
    let inner_type_width = match inner.to_string().as_str() {
        "u8" => 8,
        "u16" => 16,
        "u32" => 32,
        _ => 64,
    };

    let hex_width = usize::from(bit_width.div_ceil(4));
    let bin_width = usize::from(bit_width);
    let max_value = if bit_width == 64 {
        u64::MAX
    } else {
        (1u64 << (bit_width)) - 1
    };

    let new_method = if bit_width == inner_type_width {
        quote! {
            /// Create a new [`Self`] from a key value
            #[inline]
            #[must_use]
            pub fn new(key: #inner) -> Self {
                key.into()
            }
        }
    } else {
        quote! {
            /// Create a new [`Self`] from a key value
            #[inline]
            #[must_use]
            pub fn new(key: #inner) -> Result<Self, #error_type> {
                key.try_into()
            }
        }
    };

    quote! {
        impl #name {
            /// The bit width of this type
            pub const BIT_WIDTH: u8 = #bit_width;

            /// Minimum value for this bit width
            pub const MIN: #inner = 0;

            /// Maximum value for this bit width
            pub const MAX: #inner = #max_value;

            #new_method

            /// Convert to hex string (formatted for the bit width)
            #[must_use]
            pub fn to_hex(self) -> String {
                let value = self.0 & Self::MAX;
                format!("{:0width$X}", value, width = #hex_width)
            }

            /// Convert to binary string (full bit width with leading zeros)
            #[must_use]
            pub fn to_bin(self) -> String {
                let value = self.0 & Self::MAX;
                format!("{:0width$b}", value, width = #bin_width)
            }

            /// Check if all bits are set
            #[inline]
            #[must_use]
            pub const fn all(self) -> bool {
                self.0 == Self::MAX
            }

            /// Check if any bit is set
            #[inline]
            #[must_use]
            pub const fn any(self) -> bool {
                self.0 != 0
            }

            /// Count the number of set bits
            #[inline]
            #[must_use]
            pub const fn count_ones(self) -> u32 {
                self.0.count_ones()
            }

            /// Count the number of zero bits within the constrained bit width
            #[must_use]
            pub const fn count_zeros(self) -> u32 {
                let value = self.0 as #inner & Self::MAX;
                let ones_count = self.count_ones();
                (Self::BIT_WIDTH as u32) - ones_count
            }

            /// Reverse the bit pattern within the constrained bit width
            #[must_use]
            pub const fn reverse_bits(self) -> Self {
                let value = self.0 as #inner & Self::MAX;
                let reversed = value.reverse_bits();
                // Mask and shift to keep only the relevant bits
                let result = ((reversed >> (64 - Self::BIT_WIDTH)) & Self::MAX).reverse_bits()
                            >> (64 - Self::BIT_WIDTH);
                Self((result & Self::MAX) as #inner)
            }


            /// Rotate the bits left by `n` positions within the bit width
            #[must_use]
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
            #[must_use]
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
            #[must_use]
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
            #[must_use]
            pub fn trailing_zeros(self) -> u32 {
                let value = self.0 as #inner & Self::MAX;
                let reversed = self.reverse_bits();
                reversed.leading_zeros()
            }

            /// Find the number of leading one bits from the MSB within the bit width
            #[must_use]
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
            #[must_use]
            pub fn trailing_ones(self) -> u32 {
                let value = self.0 as #inner & Self::MAX;
                let reversed = self.reverse_bits();
                reversed.leading_ones()
            }

            /// Check if the value is zero within the bit width
            #[must_use]
            pub const fn is_zero(self) -> bool {
                (self.0 & Self::MAX)  == 0
            }

            /// Set a specific bit to 1 (clamped to bit width)
            #[must_use]
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
            #[must_use]
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
                    return Err(#error_type::value_out_of_range(value));
                }

                Ok(Self(value))
            }

            /// Extract a bitfield from the constrained range
            #[must_use]
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

            #[must_use]
            const fn clamp_bit_positions(self, bit: u8) -> u8 {
                if bit >= Self::BIT_WIDTH {
                    return Self::BIT_WIDTH - 1;
                }
                bit
            }
        }
    }
}

fn generate_bitwise_wrapper(info: &Struct) -> TokenStream {
    let name = &info.name;

    quote! {
        impl std::ops::Not for #name {
            type Output = Self;
            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }

        impl std::ops::BitAnd for #name {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl std::ops::BitAnd<&Self> for #name {
            type Output = Self;
            fn bitand(self, rhs: &Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl std::ops::BitAndAssign for #name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl std::ops::BitAndAssign<&Self> for #name {
            fn bitand_assign(&mut self, rhs: &Self) {
                self.0 &= rhs.0;
            }
        }

        impl std::ops::BitOr for #name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl std::ops::BitOr<&Self> for #name {
            type Output = Self;
            fn bitor(self, rhs: &Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl std::ops::BitOrAssign for #name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl std::ops::BitOrAssign<&Self> for #name {
            fn bitor_assign(&mut self, rhs: &Self) {
                self.0 |= rhs.0;
            }
        }

        impl std::ops::BitXor for #name {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }

        impl std::ops::BitXor<&Self> for #name {
            type Output = Self;
            fn bitxor(self, rhs: &Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }

        impl std::ops::BitXorAssign for #name {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }

        impl std::ops::BitXorAssign<&Self> for #name {
            fn bitxor_assign(&mut self, rhs: &Self) {
                self.0 ^= rhs.0;
            }
        }

        impl std::ops::Shl<u32> for #name {
            type Output = Self;
            fn shl(self, rhs: u32) -> Self {
                Self(self.0 << rhs)
            }
        }

        impl std::ops::ShlAssign<u32> for #name {
            fn shl_assign(&mut self, rhs: u32) {
                self.0 <<= rhs;
            }
        }

        impl std::ops::Shr<u32> for #name {
            type Output = Self;
            fn shr(self, rhs: u32) -> Self::Output {
                Self(self.0 >> rhs)
            }
        }

        impl std::ops::ShrAssign<u32> for #name {
            fn shr_assign(&mut self, rhs: u32) {
                self.0 >>= rhs;
            }
        }
    }
}

fn generate_from_impl(
    bit_width: u8,
    target_width: u8,
    source_type: &TokenStream,
    target_type: &Ident,
    inner_type: &Ident,
) -> Option<TokenStream> {
    if bit_width < target_width {
        return None;
    }
    Some(quote! {
        impl std::convert::From<#source_type> for #target_type {
            fn from(key: #source_type) -> Self {
                Self(key as #inner_type)
            }
        }
    })
}

fn generate_try_from_impl(
    bit_width: u8,
    target_width: u8,
    source_type: &TokenStream,
    target_type: &Ident,
    inner_type: &Ident,
    error_type: &Ident,
) -> Option<TokenStream> {
    if bit_width >= target_width {
        return None;
    }
    Some(quote! {
        impl std::convert::TryFrom<#source_type> for #target_type {
            type Error = #error_type;
            fn try_from(key: #source_type) -> Result<Self, Self::Error> {
                if key > #target_type::MAX {
                    return Err(#error_type::value_out_of_range(key));
                }
                Ok(Self(key as #inner_type))
            }
        }
    })
}
