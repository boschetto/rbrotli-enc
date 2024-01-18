// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{safe_arch, x86_64::*, CheckLengthsSimd, CheckPow2, CheckPow2Size, CheckSameSize};
use bounded_utils::{BoundedSlice, BoundedU32, BoundedU8, BoundedUsize};
use zerocopy::{AsBytes, FromBytes};

const AVX_VECTOR_SIZE: usize = 32;

#[inline]
#[target_feature(enable = "avx")]
#[safe_arch]
pub fn _mm256_load<T: AsBytes, const SLICE_BOUND: usize, const START_BOUND: usize>(
    data: &BoundedSlice<T, SLICE_BOUND>,
    start: BoundedUsize<START_BOUND>,
) -> __m256i {
    let _ = CheckLengthsSimd::<T, SLICE_BOUND, START_BOUND, AVX_VECTOR_SIZE>::CHECK_GE;
    let _ = CheckPow2Size::<T, AVX_VECTOR_SIZE>::IS_POW2;
    // SAFETY: safety ensured by target_feature_11 + the above length check, which ensures that a
    // full vector can still be read after `start`.
    unsafe { _mm256_loadu_si256(data.get_slice().as_ptr().add(start.get()) as *const _) }
}

#[inline]
#[target_feature(enable = "avx")]
#[safe_arch]
pub fn _mm256_store<T: FromBytes, const SLICE_BOUND: usize, const START_BOUND: usize>(
    data: &mut BoundedSlice<T, SLICE_BOUND>,
    start: BoundedUsize<START_BOUND>,
    value: __m256i,
) {
    let _ = CheckLengthsSimd::<T, SLICE_BOUND, START_BOUND, AVX_VECTOR_SIZE>::CHECK_GE;
    let _ = CheckPow2Size::<T, AVX_VECTOR_SIZE>::IS_POW2;
    // SAFETY: safety ensured by target_feature_11 + the above length check, which ensures that a
    // full vector can still be read after `start`.
    unsafe {
        _mm256_storeu_si256(
            data.get_slice_mut().as_mut_ptr().add(start.get()) as *mut _,
            value,
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
#[safe_arch]
pub fn _mm256_store_masked_u8<
    const SLICE_BOUND: usize,
    const START_BOUND: usize,
    const VALUE_BOUND: usize,
>(
    data: &mut BoundedSlice<BoundedU8<VALUE_BOUND>, SLICE_BOUND>,
    start: BoundedUsize<START_BOUND>,
    value: __m256i,
) {
    let _ = CheckLengthsSimd::<u8, SLICE_BOUND, START_BOUND, AVX_VECTOR_SIZE>::CHECK_GE;
    let _ = CheckPow2::<VALUE_BOUND>::IS_POW2;
    // SAFETY: safety ensured by target_feature_11 + the above length check, which ensures that a
    // full vector can still be read after `start`; the `BoundedU8` invariant is upheld by the
    // `_mm256_and_si256` operation.
    unsafe {
        _mm256_storeu_si256(
            data.get_slice_mut().as_mut_ptr().add(start.get()) as *mut _,
            _mm256_and_si256(_mm256_set1_epi8(VALUE_BOUND as i8 - 1), value),
        );
    }
}

#[inline]
#[target_feature(enable = "sse2")]
#[safe_arch]
pub fn _mm256_store_masked_u32<
    const SLICE_BOUND: usize,
    const START_BOUND: usize,
    const VALUE_BOUND: usize,
>(
    data: &mut BoundedSlice<BoundedU32<VALUE_BOUND>, SLICE_BOUND>,
    start: BoundedUsize<START_BOUND>,
    value: __m256i,
) {
    let _ = CheckLengthsSimd::<u32, SLICE_BOUND, START_BOUND, AVX_VECTOR_SIZE>::CHECK_GE;
    let _ = CheckPow2::<VALUE_BOUND>::IS_POW2;
    // SAFETY: safety ensured by target_feature_11 + the above length check, which ensures that a
    // full vector can still be read after `start`; the `BoundedU32` invariant is upheld by the
    // `_mm256_and_si256` operation.
    unsafe {
        _mm256_storeu_si256(
            data.get_slice_mut().as_mut_ptr().add(start.get()) as *mut _,
            _mm256_and_si256(_mm256_set1_epi32(VALUE_BOUND as i32 - 1), value),
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
#[safe_arch]
pub fn _mm256_masked_i32gather<T: AsBytes, const SCALE: i32, const ARRAY_BOUND: usize>(
    slice: &BoundedSlice<T, ARRAY_BOUND>,
    offsets: __m256i,
) -> __m256i {
    let _ = CheckPow2::<ARRAY_BOUND>::IS_POW2;
    let _ = CheckSameSize::<T, SCALE>::SAME_SIZE;
    // SAFETY: safety ensured by target_feature_11 + the _mm256_and_si256 operation that
    // ensure no OOB read can happen.
    unsafe {
        _mm256_i32gather_epi32::<SCALE>(
            slice.get_slice().as_ptr().cast(),
            _mm256_and_si256(offsets, _mm256_set1_epi32(ARRAY_BOUND as i32 - 1)),
        )
    }
}

const SSE_VECTOR_SIZE: usize = 16;

#[inline]
#[target_feature(enable = "sse2")]
#[safe_arch]
pub fn _mm_load<T: AsBytes, const SLICE_BOUND: usize, const START_BOUND: usize>(
    data: &BoundedSlice<T, SLICE_BOUND>,
    start: BoundedUsize<START_BOUND>,
) -> __m128i {
    let _ = CheckLengthsSimd::<T, SLICE_BOUND, START_BOUND, SSE_VECTOR_SIZE>::CHECK_GE;
    let _ = CheckPow2Size::<T, SSE_VECTOR_SIZE>::IS_POW2;
    // SAFETY: safety ensured by target_feature_11 + the above length check, which ensures that a
    // full vector can still be read after `start`.
    unsafe { _mm_loadu_si128(data.get_slice().as_ptr().add(start.get()) as *const _) }
}

#[inline]
#[target_feature(enable = "sse2")]
#[safe_arch]
pub fn _mm_store<T: FromBytes, const SLICE_BOUND: usize, const START_BOUND: usize>(
    data: &mut BoundedSlice<T, SLICE_BOUND>,
    start: BoundedUsize<START_BOUND>,
    value: __m128i,
) {
    let _ = CheckLengthsSimd::<T, SLICE_BOUND, START_BOUND, SSE_VECTOR_SIZE>::CHECK_GE;
    let _ = CheckPow2Size::<T, SSE_VECTOR_SIZE>::IS_POW2;
    // SAFETY: safety ensured by target_feature_11 + the above length check, which ensures that a
    // full vector can still be read after `start`.
    unsafe {
        _mm_storeu_si128(
            data.get_slice_mut().as_mut_ptr().add(start.get()) as *mut _,
            value,
        );
    }
}

#[inline]
#[target_feature(enable = "sse2")]
#[safe_arch]
pub fn _mm_store_masked_u8<
    const SLICE_BOUND: usize,
    const START_BOUND: usize,
    const VALUE_BOUND: usize,
>(
    data: &mut BoundedSlice<BoundedU8<VALUE_BOUND>, SLICE_BOUND>,
    start: BoundedUsize<START_BOUND>,
    value: __m128i,
) {
    let _ = CheckLengthsSimd::<u8, SLICE_BOUND, START_BOUND, SSE_VECTOR_SIZE>::CHECK_GE;
    let _ = CheckPow2::<VALUE_BOUND>::IS_POW2;
    // SAFETY: safety ensured by target_feature_11 + the above length check, which ensures that a
    // full vector can still be read after `start`; the `BoundedU8` invariant is upheld by the
    // `_mm_and_si128` operation.
    unsafe {
        _mm_storeu_si128(
            data.get_slice_mut().as_mut_ptr().add(start.get()) as *mut _,
            _mm_and_si128(_mm_set1_epi8(VALUE_BOUND as i8 - 1), value),
        );
    }
}

#[inline]
#[target_feature(enable = "sse2")]
#[safe_arch]
pub fn _mm_store_masked_u32<
    const SLICE_BOUND: usize,
    const START_BOUND: usize,
    const VALUE_BOUND: usize,
>(
    data: &mut BoundedSlice<BoundedU32<VALUE_BOUND>, SLICE_BOUND>,
    start: BoundedUsize<START_BOUND>,
    value: __m128i,
) {
    let _ = CheckLengthsSimd::<u32, SLICE_BOUND, START_BOUND, SSE_VECTOR_SIZE>::CHECK_GE;
    let _ = CheckPow2::<VALUE_BOUND>::IS_POW2;
    // SAFETY: safety ensured by target_feature_11 + the above length check, which ensures that a
    // full vector can still be read after `start`; the `BoundedU32` invariant is upheld by the
    // `_mm_and_si128` operation.
    unsafe {
        _mm_storeu_si128(
            data.get_slice_mut().as_mut_ptr().add(start.get()) as *mut _,
            _mm_and_si128(_mm_set1_epi32(VALUE_BOUND as i32 - 1), value),
        );
    }
}
