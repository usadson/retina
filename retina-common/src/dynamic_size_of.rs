// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub trait DynamicSizeOf {
    fn dynamic_size_of(&self) -> usize;
}

impl<T> DynamicSizeOf for Vec<T>
        where T: DynamicSizeOf {
    fn dynamic_size_of(&self) -> usize {
        let elements_size: usize = self.iter()
            .map(|element| element.dynamic_size_of())
            .sum();

        let rest_of_capacity = (self.capacity() - self.len())
            * std::mem::size_of::<T>();

        elements_size + rest_of_capacity
    }
}
