// https://betterprogramming.pub/learning-rust-building-a-linked-list-102bcb08f93b

// #[derive(Debug)]


// struct LinkedNode {
//     data : i32,
//     next : Option<Box<LinkedNode>>,
// }

// struct MyList {
//     size : u32,
//     head : Option<Box<LinkedNode>>,
// }
// impl LinkedNode {
//     fn push(&mut self , element :i32){
//         match self.next {
//             Some(next) => next.push(element),
//             None => self.next = Some(LinkedNode::Box::new{
//                 data : element,
//                 next : None

//             })
//         }
//     }
// }


#[derive(Debug)]
struct Node {
    element : u32,
    next : Link,
}
type Link = Option<Box<Node>>;

#[derive(Debug)]
struct LinkedList {
    head :Link,
}
// #[derive(Debug)]
// enum Link {
//     Empty,
//     NonEmpty(Box<Node>),

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {head :None}
    }

    fn push(&mut self, element: u32) {
        // let old_head = std::mem::replace(&mut self.head, None );
        let old_head = self.head.take();
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });
        self.head = Some(new_head);
        // match self.head {
        //     None => {
        //         self.head =Some(Box::new (Node{
        //             element,
        //             next : None,
        //         }))
        //     }
        //     Some(n ) => {
        //         let new_head  = Some(Box::new(Node {
        //             element,
        //             next: Some(n)
        //         }));
        //         self.head = new_head;

        //     }
            
        }

        fn pop(&mut self) -> Option<u32> {
            // let old_head = self.head.take();

           self.head.take().map(|n|{
                self.head = n.next;
                n.element

            })
            // match old_head {
            //     None => None,
            //     Some(node) => {
            //         self.head = node.next;
            //         Some(node.element)
            //     }
            // }
            // self.head.take().map(|node| {
            // self.head.take().map(|node| {
            //     self.head = node.next;
            //     node.element
            // })
        }

        fn peek(&self) -> Option<u32> {
            self.head.as_ref().map(|n| n.element)
        }

        fn size(&self) -> u32 {
            let mut count = 0;
            let mut current = &self.head;
            while let Some(node) = current {
                count += 1;
                current = &node.next;
            }
            count
        }

    }




fn main() {
    let list = Link::Some(Box::new(Node{
        element : 1001,
        next: Link::None,
    }));
    println!("{:?}" ,list );
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works(){
        let list: LinkedList = LinkedList{
            head:Some (
                Box::new(Node{
            element:1024,
            next: None,
            
        })),
        };
    }
    #[test]
    fn test_push(){
        let mut list = LinkedList::new();
        list.push(1001);
        list.push(1002);
        list.push(1003);
        assert_eq!(list.size(),3);
        

    }
}  