use super::stack::Stack;

///We require Copy in order to be able to keep elements in both Stacks.
///The alternative would be to use some special pointer but that is still pending
///to study
#[derive(Debug)]
pub struct StackWithMax<T: PartialOrd + Copy> {
    elems: Stack<T>,
    maxs: Stack<T>,
}

impl<T: PartialOrd + Copy> StackWithMax<T> {
    pub fn new() -> Self {
        StackWithMax {
            elems: Stack::new(),
            maxs: Stack::new(),
        }
    }

    //With a peek operation, this could be more efficient. But, for now,
    //it is not clear to me how to implement the peek operation. So we do it
    //like this
    pub fn push(&mut self, elem: T) {
        self.elems.push(elem);

        if let Some(max_elem) = self.maxs.pop() {
            self.maxs.push(max_elem);

            if elem >= max_elem {
                self.maxs.push(elem);
            }
        } else {
            self.maxs.push(elem);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let popped_elem = self.elems.pop()?;
        let max_elem = self.maxs.pop()?;

        if popped_elem != max_elem {
            self.maxs.push(max_elem);
        }

        Some(popped_elem)
    }

    pub fn max(&mut self) -> Option<T> {
        let maximum_element = self.maxs.pop()?;
        self.maxs.push(maximum_element);

        Some(maximum_element)
    }
}
