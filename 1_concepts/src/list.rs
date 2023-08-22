/// Common trait for list with some basic operations
pub trait List<T> {
    /// Push a value to the list's tail
    /// # Arguments
    /// * `value` - The value to push
    /// # Returns
    /// * `Option<usize>` - The index of the value in the list
    fn push(&mut self, value: T) -> Option<usize>;

    /// Pop a value from the list's tail
    /// # Returns
    /// * `Option<T>` - The value from the list's tail
    fn pop(&mut self) -> Option<T>;

    /// Get a value from the list by index
    /// # Arguments
    /// * `index` - The index of the value to get
    /// # Returns
    /// * `Option<T>` - The value from the list
    fn get(&self, index: usize) -> Option<T>;
    
    /// Remove a value from the list by index
    /// # Arguments
    /// * `index` - The index of the value to remove
    /// # Returns
    /// * `Option<T>` - The value from the list
    fn remove(&mut self, index: usize) -> Option<T>;

    /// Update a value in the list by index
    /// # Arguments
    /// * `index` - The index of the value to update
    /// * `value` - The new value
    /// # Returns
    /// * `Option<T>` - The old value
    fn update(&mut self, index: usize, value: T) -> Option<T>;

    /// Clear the list. This should remove all values from the list
    /// # Returns
    /// * `()` - Nothing
    fn clear(&mut self);
}

pub trait AdvancedList<T> {
        
}
