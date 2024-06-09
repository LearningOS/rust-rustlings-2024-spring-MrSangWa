/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd + Clone + std::cmp::PartialOrd + std::fmt::Debug>(array: &mut [T]){
	let mut stack = vec![(0, array.len() - 1)];

    while let Some((l, r)) = stack.pop() {
        if l < r {
            let mid = array[l + r + 1 >> 1].clone();
            let (mut p, mut q) = (l, r);
            while p < q {
                while p < q && array[p] < mid {
                    p += 1;
                }
                while p < q && array[q] > mid {
                    q -= 1;
                }
                if p < q {
                    unsafe {
                        let arr = array.as_mut_ptr();
                        std::mem::swap(&mut (*arr.add(p)), &mut (*arr.add(q)));
                    }
                }
            }
            println!("{}, {}, {}, {}, {:?}", l, r, p, q, array);
            if (p > 0) { stack.push((l, p - 1)); }
            stack.push((p, r));
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}