/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>,
    size:usize
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new(),
            size:0
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        if self.q1.size() != 0 {
            self.q1.enqueue(elem);
        }else {
            self.q2.enqueue(elem);
        }
        self.size +=1;
    }
    // pub fn pop(&mut self) -> Result<T, &str> {
    //     //TODO
    //     // if self.size == 0 {
    //     //     return Err("Stack is empty");
    //     // }
	// 	// Err("Stack is empty")
    //    //  while self.q1.size() > 0 {
    //    //     let num = self.q1.dequeue()?;
    //    //      if self.q1.size() !=0 {
    //    //          self.q2.enqueue(num);
    //    //      }
    //    //      return Ok(num);
    //    //  }
    //    //  while self.q2.size() > 0 {
    //    //      let num = self.q2.dequeue()?;
    //    //      if self.q2.size() !=0 {
    //    //          self.q1.enqueue(num);
    //    //      }
    //    //      return Ok(num);
    //    //  }
    //    //
    //    //  self.size -=1;
    //    // Err("Stack is empty")
    // }
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.size == 0 {
            return Err("Stack is empty");
        }

        // 确定主队列和辅助队列
        let (main_q, aux_q) = if !self.q1.is_empty() {
            (&mut self.q1, &mut self.q2)
        } else {
            (&mut self.q2, &mut self.q1)
        };

        // 转移元素，直到主队列只剩一个元素
        while main_q.size() > 1 {
            let elem = main_q.dequeue().unwrap();
            aux_q.enqueue(elem);
        }

        // 弹出最后一个元素并更新栈大小
        let popped = main_q.dequeue().unwrap();
        self.size -= 1;

        Ok(popped)
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        if self.size == 0 {
            return true
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}