use super::{BoolTensor, Device, FloatTensor, IntElem, IntTensor};
use crate::{backend::Backend, tensor::Shape, Data, ElementConversion, Int};
use crate::{tensor::api::chunk, tensor::api::narrow};
use alloc::vec::Vec;
use burn_common::reader::Reader;
use core::ops::Range;
use num_traits::ToPrimitive;

/// Int Tensor API for basic and numeric operations, see [tensor](crate::Tensor)
/// for documentation on each function.
pub trait IntTensorOps<B: Backend> {
    /// Creates a new int tensor.
    ///
    /// # Arguments
    ///
    /// * `shape` - The shape of the tensor.
    /// * `device` - The device to create the tensor on.
    ///
    /// # Returns
    ///
    /// The integer tensor with the given shape.
    fn int_empty<const D: usize>(shape: Shape<D>, device: &Device<B>) -> IntTensor<B, D>;

    /// Returns the shape of the tensor.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor.
    ///
    /// # Returns
    ///
    /// The shape of the tensor.
    fn int_shape<const D: usize>(tensor: &IntTensor<B, D>) -> Shape<D>;

    /// Converts the tensor to a data structure.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor.
    ///
    /// # Returns
    ///
    /// The data structure with the tensor's data.
    fn int_into_data<const D: usize>(tensor: IntTensor<B, D>) -> Reader<Data<IntElem<B>, D>>;

    /// Gets the data from the tensor.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor.
    ///
    /// # Returns
    ///
    /// The data cloned from the data structure.
    fn int_to_data<const D: usize>(tensor: &IntTensor<B, D>) -> Reader<Data<IntElem<B>, D>> {
        Self::int_into_data(tensor.clone())
    }

    /// Creates a tensor from the data structure.
    ///
    /// # Arguments
    ///
    /// * `data` - The data structure.
    /// * `device` - The device to create the tensor on.
    ///
    /// # Returns
    ///
    /// The tensor with the data.
    fn int_from_data<const D: usize>(
        data: Data<IntElem<B>, D>,
        device: &Device<B>,
    ) -> IntTensor<B, D>;

    /// Gets the device of the tensor.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor.
    ///
    /// # Returns
    ///
    /// The device of the tensor.
    fn int_device<const D: usize>(tensor: &IntTensor<B, D>) -> Device<B>;

    /// Moves the tensor to the given device.
    fn int_to_device<const D: usize>(
        tensor: IntTensor<B, D>,
        device: &Device<B>,
    ) -> IntTensor<B, D>;

    /// Reshapes the tensor.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor.
    /// * `shape` - The new shape.
    ///
    /// # Returns
    ///
    /// The tensor with the new shape.
    fn int_reshape<const D1: usize, const D2: usize>(
        tensor: IntTensor<B, D1>,
        shape: Shape<D2>,
    ) -> IntTensor<B, D2>;

    /// Gets the element at the given indices.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor.
    /// * `indices` - The indices.
    ///
    /// # Returns
    ///
    /// The elements at the given indices.
    fn int_slice<const D1: usize, const D2: usize>(
        tensor: IntTensor<B, D1>,
        indices: [Range<usize>; D2],
    ) -> IntTensor<B, D1>;

    /// Sets the element at the given indices.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor.
    /// * `indices` - The indices.
    ///
    /// # Returns
    ///
    /// The tensor with the element at the given indices set.
    fn int_slice_assign<const D1: usize, const D2: usize>(
        tensor: IntTensor<B, D1>,
        indices: [Range<usize>; D2],
        value: IntTensor<B, D1>,
    ) -> IntTensor<B, D1>;

    /// Converts int tensor to float tensor.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor.
    ///
    /// # Returns
    ///
    /// The int tensor with the same data as the float tensor.
    fn int_into_float<const D: usize>(tensor: IntTensor<B, D>) -> FloatTensor<B, D>;

    /// Fills the tensor with values from the source tensor if the mask is true at the given
    /// indices.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor.
    /// * `mask` - The mask.
    /// * `source` - The source tensor.
    ///
    /// # Returns
    ///
    /// The tensor with the values filled.
    fn int_mask_where<const D: usize>(
        tensor: IntTensor<B, D>,
        mask: BoolTensor<B, D>,
        source: IntTensor<B, D>,
    ) -> IntTensor<B, D>;

    /// Fills the tensor with the given value if the mask is true at the given indices.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor.
    /// * `mask` - The mask.
    /// * `value` - The value.
    ///
    /// # Returns
    ///
    /// The tensor with the values filled.
    fn int_mask_fill<const D: usize>(
        tensor: IntTensor<B, D>,
        mask: BoolTensor<B, D>,
        value: IntElem<B>,
    ) -> IntTensor<B, D>;

    /// Gather elements from the tensor at the given indices.
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimension to gather from.
    /// * `tensor` - The tensor.
    /// * `indices` - The indices.
    fn int_gather<const D: usize>(
        dim: usize,
        tensor: IntTensor<B, D>,
        indices: IntTensor<B, D>,
    ) -> IntTensor<B, D>;

    /// Scatter a given value to the tensor at the given indices.
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimension to scatter to.
    /// * `tensor` - The tensor.
    /// * `indices` - The indices.
    /// * `value` - The value.
    ///
    /// # Returns
    ///
    /// The tensor with the values scattered.
    fn int_scatter<const D: usize>(
        dim: usize,
        tensor: IntTensor<B, D>,
        indices: IntTensor<B, D>,
        value: IntTensor<B, D>,
    ) -> IntTensor<B, D>;

    /// Select tensor elements along the given dimension corresponding to the given indices.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor.
    /// * `dim` - The dimension to select from.
    /// * `indices` - The indices.
    ///
    /// # Returns
    ///
    /// The tensor with the selected elements.
    fn int_select<const D: usize>(
        tensor: IntTensor<B, D>,
        dim: usize,
        indices: IntTensor<B, 1>,
    ) -> IntTensor<B, D>;

    /// Assign the selected elements along the given dimension corresponding to the given indices
    /// to the given value.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor.
    /// * `dim` - The dimension to select from.
    /// * `indices` - The indices.
    /// * `value` - The value.
    ///
    /// # Returns
    ///
    /// The tensor with the selected elements assigned to the given value.
    fn int_select_assign<const D: usize>(
        tensor: IntTensor<B, D>,
        dim: usize,
        indices: IntTensor<B, 1>,
        value: IntTensor<B, D>,
    ) -> IntTensor<B, D>;

    /// Repeats the tensor along the given dimension the given number of times.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor.
    /// * `dim` - The dimension to repeat.
    /// * `times` - The number of times to repeat.
    ///
    /// # Returns
    ///
    /// The tensor with the given dimension repeated the given number of times.
    fn int_repeat<const D: usize>(
        tensor: IntTensor<B, D>,
        dim: usize,
        times: usize,
    ) -> IntTensor<B, D> {
        let mut shape = Self::int_shape(&tensor);
        if shape.dims[dim] != 1 {
            panic!("Can only repeat dimension with dim=1");
        }
        shape.dims[dim] = times;

        let mut i = 0;
        let indices_select_all = [0; D].map(|_| {
            let start = 0;
            let end = shape.dims[i];
            i += 1;
            start..end
        });

        let mut tensor_output = Self::int_empty(shape, &Self::int_device(&tensor));
        for i in 0..times {
            let mut indices = indices_select_all.clone();
            indices[dim] = i..i + 1;
            tensor_output = Self::int_slice_assign(tensor_output, indices, tensor.clone());
        }

        tensor_output
    }

    /// Concatenates the given tensors along the given dimension.
    ///
    /// # Arguments
    ///
    /// * `tensors` - The tensors.
    /// * `dim` - The dimension to concatenate along.
    ///
    /// # Returns
    ///
    /// The concatenated tensor.
    fn int_cat<const D: usize>(tensors: Vec<IntTensor<B, D>>, dim: usize) -> IntTensor<B, D>;

    /// Elementwise equality comparison.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side tensor.
    ///
    /// # Returns
    ///
    /// The boolean tensor with the result of the comparison.
    fn int_equal<const D: usize>(lhs: IntTensor<B, D>, rhs: IntTensor<B, D>) -> BoolTensor<B, D>;

    /// Elementwise equality comparison with a scalar.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side scalar.
    ///
    /// # Returns
    ///
    /// The boolean tensor with the result of the comparison.
    fn int_equal_elem<const D: usize>(lhs: IntTensor<B, D>, rhs: IntElem<B>) -> BoolTensor<B, D>;

    /// Elementwise greater than comparison.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side tensor.
    ///
    /// # Returns
    ///
    /// The boolean tensor with the result of the comparison.
    fn int_greater<const D: usize>(lhs: IntTensor<B, D>, rhs: IntTensor<B, D>) -> BoolTensor<B, D>;

    /// Elementwise greater than comparison with a scalar.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side scalar.
    ///
    /// # Returns
    ///
    /// The boolean tensor with the result of the comparison.
    fn int_greater_elem<const D: usize>(lhs: IntTensor<B, D>, rhs: IntElem<B>) -> BoolTensor<B, D>;

    /// Elementwise greater than or equal comparison.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side tensor.
    ///
    /// # Returns
    ///
    /// The boolean tensor with the result of the comparison.
    fn int_greater_equal<const D: usize>(
        lhs: IntTensor<B, D>,
        rhs: IntTensor<B, D>,
    ) -> BoolTensor<B, D>;

    /// Elementwise greater than or equal comparison with a scalar.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side scalar.
    ///
    /// # Returns
    ///
    /// The boolean tensor with the result of the comparison.
    fn int_greater_equal_elem<const D: usize>(
        lhs: IntTensor<B, D>,
        rhs: IntElem<B>,
    ) -> BoolTensor<B, D>;

    /// Elementwise less than comparison.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side tensor.
    ///
    /// # Returns
    ///
    /// The boolean tensor with the result of the comparison.
    fn int_lower<const D: usize>(lhs: IntTensor<B, D>, rhs: IntTensor<B, D>) -> BoolTensor<B, D>;

    /// Elementwise less than comparison with a scalar.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side scalar.
    ///
    /// # Returns
    ///
    /// The boolean tensor with the result of the comparison.
    fn int_lower_elem<const D: usize>(lhs: IntTensor<B, D>, rhs: IntElem<B>) -> BoolTensor<B, D>;

    /// Elementwise less than or equal comparison.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side tensor.
    ///
    /// # Returns
    ///
    /// The boolean tensor with the result of the comparison.
    fn int_lower_equal<const D: usize>(
        lhs: IntTensor<B, D>,
        rhs: IntTensor<B, D>,
    ) -> BoolTensor<B, D>;

    /// Elementwise less than or equal comparison with a scalar.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side scalar.
    ///
    /// # Returns
    ///
    /// The boolean tensor with the result of the comparison.
    fn int_lower_equal_elem<const D: usize>(
        lhs: IntTensor<B, D>,
        rhs: IntElem<B>,
    ) -> BoolTensor<B, D>;

    // ====  NUMERIC ==== //

    /// Elementwise addition.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side tensor.
    ///
    /// # Returns
    ///
    /// The result of the addition.
    fn int_add<const D: usize>(lhs: IntTensor<B, D>, rhs: IntTensor<B, D>) -> IntTensor<B, D>;

    /// Elementwise addition with a scalar.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side scalar.
    ///
    /// # Returns
    ///
    /// The result of the addition.
    fn int_add_scalar<const D: usize>(lhs: IntTensor<B, D>, rhs: IntElem<B>) -> IntTensor<B, D>;

    /// Elementwise power with a IntTensor.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side IntTensor.
    /// * `rhs` - The right hand side IntTensor.
    ///
    /// # Returns
    ///
    /// The elements of `lhs` raised to the power of the elements of `rhs`.
    fn int_powi<const D: usize>(lhs: IntTensor<B, D>, rhs: IntTensor<B, D>) -> IntTensor<B, D> {
        B::float_into_int(B::float_powf(
            B::int_into_float(lhs),
            B::int_into_float(rhs),
        ))
    }

    /// Elementwise power with a floatTensor.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side floatTensor.
    ///
    /// # Returns
    ///
    /// The elements of `lhs` raised to the value of `rhs`. Result is an IntTensor.
    fn int_powf<const D: usize>(lhs: IntTensor<B, D>, rhs: FloatTensor<B, D>) -> IntTensor<B, D> {
        B::float_into_int(B::float_powf(B::int_into_float(lhs), rhs))
    }

    /// Elementwise power with a scalar.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side scalar.
    ///
    /// # Returns
    ///
    /// The elements of `lhs` raised to the value of `rhs`.
    fn int_powi_scalar<const D: usize>(lhs: IntTensor<B, D>, rhs: IntElem<B>) -> IntTensor<B, D> {
        B::float_into_int(B::float_powf_scalar(
            B::int_into_float(lhs),
            rhs.to_f32().unwrap(),
        ))
    }

    /// Elementwise power with a floatTensor.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side scalar.
    ///
    /// # Returns
    ///
    /// The elements of `lhs` raised to the value of `rhs`. Result is an IntTensor.
    fn int_powf_scalar<const D: usize>(lhs: IntTensor<B, D>, rhs: f32) -> IntTensor<B, D> {
        B::float_into_int(B::float_powf_scalar(B::int_into_float(lhs), rhs))
    }

    /// Clamps a tensor under a minimum value.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to clamp.
    /// * `min` - The minimum value.
    ///
    /// # Returns
    ///
    /// The clamped tensor.
    fn int_clamp_min<const D: usize>(tensor: IntTensor<B, D>, min: IntElem<B>) -> IntTensor<B, D> {
        let mask = Self::int_lower_elem(tensor.clone(), min);
        Self::int_mask_fill(tensor, mask, min)
    }

    /// Clamps a tensor over a maximum value.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to clamp.
    /// * `max` - The maximum value.
    ///
    /// # Returns
    ///
    /// The clamped tensor.
    fn int_clamp_max<const D: usize>(tensor: IntTensor<B, D>, max: IntElem<B>) -> IntTensor<B, D> {
        let mask = Self::int_greater_elem(tensor.clone(), max);
        Self::int_mask_fill(tensor, mask, max)
    }

    /// Clamps a tensor between a minimum and maximum value.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to clamp.
    /// * `min` - The minimum value.
    /// * `max` - The maximum value.
    ///
    /// # Returns
    ///
    /// The clamped tensor.
    fn int_clamp<const D: usize>(
        tensor: IntTensor<B, D>,
        min: IntElem<B>,
        max: IntElem<B>,
    ) -> IntTensor<B, D> {
        Self::int_clamp_min(Self::int_clamp_max(tensor, max), min)
    }

    /// Elementwise subtraction.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side tensor.
    ///
    /// # Returns
    ///
    /// The result of the subtraction.
    fn int_sub<const D: usize>(lhs: IntTensor<B, D>, rhs: IntTensor<B, D>) -> IntTensor<B, D>;

    /// Elementwise subtraction with a scalar.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side scalar.
    ///
    /// # Returns
    ///
    /// The result of the subtraction.
    fn int_sub_scalar<const D: usize>(lhs: IntTensor<B, D>, rhs: IntElem<B>) -> IntTensor<B, D>;

    /// Elementwise multiplication.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side tensor.
    ///
    /// # Returns
    ///
    /// The result of the multiplication.
    fn int_mul<const D: usize>(lhs: IntTensor<B, D>, rhs: IntTensor<B, D>) -> IntTensor<B, D>;

    /// Elementwise multiplication with a scalar.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side scalar.
    ///
    /// # Returns
    ///
    /// The result of the multiplication.
    fn int_mul_scalar<const D: usize>(lhs: IntTensor<B, D>, rhs: IntElem<B>) -> IntTensor<B, D>;

    /// Elementwise division.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side tensor.
    ///
    /// # Returns
    ///
    /// The result of the division.
    fn int_div<const D: usize>(lhs: IntTensor<B, D>, rhs: IntTensor<B, D>) -> IntTensor<B, D>;

    /// Elementwise division with a scalar.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left hand side tensor.
    /// * `rhs` - The right hand side scalar.
    ///
    /// # Returns
    ///
    /// The result of the division.
    fn int_div_scalar<const D: usize>(lhs: IntTensor<B, D>, rhs: IntElem<B>) -> IntTensor<B, D>;

    /// Elementwise negation.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to negate.
    ///
    /// # Returns
    ///
    /// The negated tensor.
    fn int_neg<const D: usize>(tensor: IntTensor<B, D>) -> IntTensor<B, D> {
        Self::int_mul_scalar(tensor, (-1.0).elem::<IntElem<B>>())
    }

    /// Creates a tensor of zeros.
    ///
    /// # Arguments
    ///
    /// * `shape` - The shape of the tensor.
    /// * `device` - The device to create the tensor on.
    ///
    /// # Returns
    ///
    /// The tensor of zeros.
    fn int_zeros<const D: usize>(shape: Shape<D>, device: &Device<B>) -> IntTensor<B, D>;

    /// Creates a tensor of ones.
    ///
    /// # Arguments
    ///
    /// * `shape` - The shape of the tensor.
    /// * `device` - The device to create the tensor on.
    ///
    /// # Returns
    ///
    /// The tensor of ones.
    fn int_ones<const D: usize>(shape: Shape<D>, device: &Device<B>) -> IntTensor<B, D>;

    /// Creates a tensor filled with given value.
    ///
    /// # Arguments
    ///
    /// * `shape` - The shape of the tensor.
    /// * `fill_value` - The value with which to fill the tensor.
    /// * `device` - The device to create the tensor on.
    ///
    /// # Returns
    ///
    /// The tensor filled with given value
    fn int_full<const D: usize>(
        shape: Shape<D>,
        fill_value: IntElem<B>,
        device: &Device<B>,
    ) -> IntTensor<B, D> {
        Self::int_add_scalar(Self::int_zeros(shape, device), fill_value)
    }

    /// Sums all elements in the tensor.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to sum.
    ///
    /// # Returns
    ///
    /// The sum of all elements in the tensor.
    fn int_sum<const D: usize>(tensor: IntTensor<B, D>) -> IntTensor<B, 1>;

    /// Sums all elements in the tensor along a dimension.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to sum.
    /// * `dim` - The dimension to sum along.
    ///
    /// # Returns
    ///
    /// The sum of all elements in the tensor along the dimension.
    fn int_sum_dim<const D: usize>(tensor: IntTensor<B, D>, dim: usize) -> IntTensor<B, D>;

    /// Computes the mean of all elements in the tensor.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to compute the mean of.
    ///
    /// # Returns
    ///
    /// The mean of all elements in the tensor.
    fn int_mean<const D: usize>(tensor: IntTensor<B, D>) -> IntTensor<B, 1> {
        let num_elems = B::int_shape(&tensor).num_elements();
        B::int_div_scalar(B::int_sum(tensor), (num_elems as i64).elem())
    }

    /// Computes the mean of all elements in the tensor along a dimension.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to compute the mean of.
    ///
    /// # Returns
    ///
    /// The mean of all elements in the tensor along the dimension.
    fn int_mean_dim<const D: usize>(tensor: IntTensor<B, D>, dim: usize) -> IntTensor<B, D>;

    /// Gets the indices of the maximum elements along a dimension.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to get the maximum indices of.
    /// * `dim` - The dimension to get the maximum indices along.
    ///
    /// # Returns
    ///
    /// The indices of the maximum elements along the dimension.
    fn int_argmax<const D: usize>(tensor: IntTensor<B, D>, dim: usize) -> IntTensor<B, D>;

    /// Gets the indices of the minimum elements along a dimension.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to get the minimum indices of.
    /// * `dim` - The dimension to get the minimum indices along.
    ///
    /// # Returns
    ///
    /// The indices of the minimum elements along the dimension.
    fn int_argmin<const D: usize>(tensor: IntTensor<B, D>, dim: usize) -> IntTensor<B, D>;

    /// Gets the maximum element in the tensor.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to get the maximum element of.
    ///
    /// # Returns
    ///
    /// The maximum element in the tensor.
    fn int_max<const D: usize>(tensor: IntTensor<B, D>) -> IntTensor<B, 1> {
        let shape = B::int_shape(&tensor);
        let tensor = B::int_reshape(tensor, Shape::new([shape.num_elements()]));

        B::int_max_dim(tensor, 0)
    }

    /// Gets the maximum element in the tensor along a dimension.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to get the maximum element of.
    /// * `dim` - The dimension to get the maximum element along.
    ///
    /// # Returns
    ///
    /// The maximum element in the tensor along the dimension.
    fn int_max_dim<const D: usize>(tensor: IntTensor<B, D>, dim: usize) -> IntTensor<B, D> {
        let index = B::int_argmax(tensor.clone(), dim);

        B::int_gather(D - 1, tensor, index)
    }

    /// Gets the maximum elements and corresponding indices along a dimension.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to get the maximum elements and indices of.
    /// * `dim` - The dimension to get the maximum elements and indices along.
    ///
    /// # Returns
    ///
    /// The maximum elements and corresponding indices along the dimension.
    fn int_max_dim_with_indices<const D: usize>(
        tensor: IntTensor<B, D>,
        dim: usize,
    ) -> (IntTensor<B, D>, IntTensor<B, D>) {
        let index = B::int_argmax(tensor.clone(), dim);
        let values = B::int_gather(D - 1, tensor, index.clone());

        (values, index)
    }

    /// Gets the minimum element in the tensor.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to get the minimum element of.
    ///
    /// # Returns
    ///
    /// The minimum element in the tensor.
    fn int_min<const D: usize>(tensor: IntTensor<B, D>) -> IntTensor<B, 1> {
        let shape = B::int_shape(&tensor);
        let tensor = B::int_reshape(tensor, Shape::new([shape.num_elements()]));

        B::int_min_dim(tensor, 0)
    }

    /// Gets the minimum elements in the tensor along a dimension.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to get the minimum element of.
    /// * `dim` - The dimension to get the minimum element along.
    ///
    /// # Returns
    ///
    /// The minimum element in the tensor along the dimension.
    fn int_min_dim<const D: usize>(tensor: IntTensor<B, D>, dim: usize) -> IntTensor<B, D> {
        let index = B::int_argmin(tensor.clone(), dim);

        B::int_gather(D - 1, tensor, index)
    }

    /// Gets the minimum elements and corresponding indices along a dimension.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to get the minimum elements and indices of.
    /// * `dim` - The dimension to get the minimum elements and indices along.
    ///
    /// # Returns
    ///
    /// The minimum elements and corresponding indices along the dimension.
    fn int_min_dim_with_indices<const D: usize>(
        tensor: IntTensor<B, D>,
        dim: usize,
    ) -> (IntTensor<B, D>, IntTensor<B, D>) {
        let indices = B::int_argmin(tensor.clone(), dim);
        let values = B::int_gather(D - 1, tensor, indices.clone());

        (values, indices)
    }

    /// Returns a new tensor with absolute values.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to take absolute value of.
    ///
    /// # Returns
    ///
    /// A tensor with the same shape as `tensor` with absolute values.
    fn int_abs<const D: usize>(tensor: IntTensor<B, D>) -> IntTensor<B, D>;

    /// Transposes an int tensor.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to transpose.
    ///
    /// # Returns
    ///
    /// The transposed tensor.
    fn int_transpose<const D: usize>(tensor: IntTensor<B, D>) -> IntTensor<B, D> {
        Self::int_swap_dims(tensor, D - 2, D - 1)
    }

    /// Swaps two dimensions of an int tensor.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to swap the dimensions of.
    /// * `dim1` - The first dimension to swap.
    /// * `dim2` - The second dimension to swap.
    ///
    /// # Returns
    ///
    /// The tensor with the dimensions swapped.
    fn int_swap_dims<const D: usize>(
        tensor: IntTensor<B, D>,
        dim1: usize,
        dim2: usize,
    ) -> IntTensor<B, D>;

    /// Returns a new tensor with the given dimension narrowed to the given range.
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimension along which the tensor will be narrowed.
    /// * `start` - The starting point of the given range.
    /// * `length` - The ending point of the given range.
    /// # Panics
    ///
    /// - If the dimension is greater than the number of dimensions of the tensor.
    /// - If the given range exceeds the number of elements on the given dimension.
    ///
    /// # Returns
    ///
    /// A new tensor with the given dimension narrowed to the given range.
    fn int_narrow<const D: usize>(
        tensor: IntTensor<B, D>,
        dim: usize,
        start: usize,
        length: usize,
    ) -> IntTensor<B, D> {
        narrow::<B, D, Int>(tensor, dim, start, length)
    }

    /// Split the tensor along the given dimension into chunks.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor.
    /// * `chunks` - The number of chunks to be produced
    /// * `times` - The dimension along which the tensor will be split.
    ///
    /// # Returns
    ///
    /// A vectors of tensors
    ///
    fn int_chunk<const D: usize>(
        tensor: IntTensor<B, D>,
        chunks: usize,
        dim: usize,
    ) -> Vec<IntTensor<B, D>> {
        chunk::<B, D, Int>(tensor, chunks, dim)
    }

    /// Creates a new tensor with values from the given range with the given step size.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of values.
    /// * `step` - The step size.
    /// * `device` - The device to create the tensor on.
    ///
    /// # Returns
    ///
    /// The tensor with the given values.
    fn int_arange_step(range: Range<i64>, step: usize, device: &Device<B>) -> IntTensor<B, 1> {
        let value = range
            .step_by(step)
            .map(|i| i.elem())
            .collect::<Vec<IntElem<B>>>();
        let shape = Shape::new([value.len()]);
        let data = Data::new(value, shape);
        B::int_from_data(data, device)
    }

    /// Creates a new tensor with values from the given range.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of values.
    /// * `device` - The device to create the tensor on.
    ///
    /// # Returns
    ///
    /// The tensor with the given values.
    ///
    /// # Remarks
    ///
    /// Uses `arange_step` with a step size of 1 under the hood.
    fn int_arange(range: Range<i64>, device: &Device<B>) -> IntTensor<B, 1> {
        Self::int_arange_step(range, 1, device)
    }
}