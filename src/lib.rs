// I can't tell if most of this works, yet, there was a lot of heavy-lifting done by copilot
// almost definitely needs error handling, and probably some other stuff

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

        let mut data = Vec::with_capacity(width * height);
        
        match initial_data {
            ConstructWith::Singleton(value) => {
                for _ in 0..width * height {
                    data.push(Some(value.clone()));
                }
            },
            
            ConstructWith::Collection(values) => {
                if row_major {
                    for value in values {
                        data.push(Some(value));
                    }
                } else {
                    // TODO: does this work? i have no idea, it's untested
                    for row in 0..height {
                        for column in 0..width {
                            let index: usize = row * width + column;
                            data.push(Some(values[index].clone()));
                        }
                    }
                }
            }
        }

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
            let value = self.array2.data[self.current_index].clone();
            self.current_index += 1;
            value
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

    // copilot wrote this, but I modified it
    fn next(&mut self) -> Option<T> {
        if self.current_row < self.array2.height {
            let value = self.array2.data[self.current_row * self.array2.width + self.current_col].clone();
            self.current_row += 1;
            value
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
    // idk what this is
    //use super::*;


}
