// I can't tell if most of this works, yet, there was a lot of heavy-lifting done by copilot
// almost definitely needs error handling, and probably some other stuff

// tells the array2 constructor how to fill the array2 (with a single value or from a vec) 
pub enum ConstructWith<T> {
    Singleton(T),
    Collection(Vec<T>)
}

// row_major may become obsolete
pub struct Array2<T> {
    pub data: Vec<Option<T>>,
    pub width: usize,
    pub height: usize,
}

// the compiler lets this code run, but we may want to take a second look
impl<T: Clone> Array2<T> {

    // creates a new Array2 with the given width and height, and fills it with the given initial data (constructor)
    pub fn new(width: usize, height: usize, initial_data: ConstructWith<T>, row_major: bool) -> Array2<T> {

        // stores the actual data in the array2
        let mut data = Vec::with_capacity(width * height);
        
        // fills the array2 with the given initial data
        match initial_data {

            // fills data with single value if ConstructWith::Singleton
            ConstructWith::Singleton(value) => {
                for _ in 0..width * height {
                    data.push(Some(value.clone()));
                }
            },
            
            // fills data with values from a vec if ConstructWith::Collection
            ConstructWith::Collection(values) => {

                // fills data with values from a vec in row major form
                if row_major {
                    for value in values {
                        data.push(Some(value));
                    }

                // fills data with values from a vec in column major form
                } else {
                    for row in 0..height {
                        for column in 0..width {
                            let index: usize = row * width + column;
                            data.push(Some(values[index].clone()));
                        }
                    }
                }
            }
        }

        // returns the new Array2
        Array2 {
            data,
            width,
            height,
        }
    }

    // gets the value at the given column and row
    pub fn get(&self, column: usize, row: usize) -> Option<T> {
        let index: usize = row * self.width + column;
        self.data[index].clone()
    }

    // sets the value at the given column and row
    pub fn set(&mut self, column: usize, row: usize, value: T) {
        let index: usize = row * self.width + column;
        self.data[index] = Some(value);
    }

    // row major form iterator
    pub fn iter_row_major(&self) -> RowMajorIterator<T> {
        RowMajorIterator {
            array2: self,
            current_index: 0,
        }
    }

    // column major form iterator
    pub fn iter_col_major(&self) -> ColumnMajorIterator<T> {
        ColumnMajorIterator {
            array2: self,
            current_row: 0,
            current_col: 0,
        }
    }
}

// iterator for row major form
pub struct RowMajorIterator<'a, T> {
    array2: &'a Array2<T>,
    current_index: usize,
}

impl<'a, T: Clone> Iterator for RowMajorIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.current_index < self.array2.data.len() {
            let value = self.array2.data[self.current_index].clone()?;
            self.current_index += 1;
            Some(value)
        } else {
            None
        }
    }
}

// iterator for column major form
pub struct ColumnMajorIterator<'a, T> {
    array2: &'a Array2<T>,
    current_row: usize,
    current_col: usize,
}

// note: we can iterate over only one column by writing a more custom for loop
impl<'a, T: Clone> Iterator for ColumnMajorIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        
        // if we haven't reached the end of the column, return the next value
        if self.current_row < self.array2.height && self.current_col < self.array2.width {
            let value = self.array2.data[self.current_row * self.array2.width + self.current_col].clone()?;
            self.current_row += 1;
            Some(value)
        
        // if we have reached the end of the column, reset the row and increment the column
        } else {
            self.current_row = 0;
            self.current_col += 1;
            if self.current_col < self.array2.width {
                self.next()
            } else {
                None
            }
        }
    }
}


#[cfg(test)]
mod tests {
    // didn't get the chance to write real tests, but if we did, these are the ones they'd be

    // check if array2 is right size

    // check if array2 is filled with the right values

    // check if row major iterator returns the right values for a valid/invalid sudoku

    // check if column major iterator returns the right values for a valid/invalid sudoku

}