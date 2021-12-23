use coco_sys::{coco_observer_t, coco_problem_t, coco_suite_t};
use std::{
    ffi::{CStr, CString},
    ops::RangeInclusive,
    ptr,
};

pub struct Suite {
    inner: *mut coco_suite_t,
}

impl Suite {
    pub fn new(name: &str, instance: &str, options: &str) -> Option<Suite> {
        let name = CString::new(name).unwrap();
        let instance = CString::new(instance).unwrap();
        let options = CString::new(options).unwrap();

        let inner =
            unsafe { coco_sys::coco_suite(name.as_ptr(), instance.as_ptr(), options.as_ptr()) };

        if inner.is_null() {
            None
        } else {
            Some(Suite { inner })
        }
    }

    pub fn next_propblem(&mut self, observer: Option<&mut Observer>) -> Option<Problem> {
        let observer = observer.map(|o| o.inner).unwrap_or(ptr::null_mut());
        let inner = unsafe { coco_sys::coco_suite_get_next_problem(self.inner, observer) };

        if inner.is_null() {
            None
        } else {
            Some(Problem { inner })
        }
    }

    pub fn number_of_problems(&mut self) -> usize {
        unsafe {
            coco_sys::coco_suite_get_number_of_problems(self.inner)
                .try_into()
                .unwrap()
        }
    }
}

impl Drop for Suite {
    fn drop(&mut self) {
        unsafe {
            coco_sys::coco_suite_free(self.inner);
        }
    }
}

pub struct Problem {
    inner: *mut coco_problem_t,
}

impl Problem {
    pub fn evaluate_function(&mut self, x: &[f64], y: &mut [f64]) {
        assert_eq!(self.dimension(), x.len());
        assert_eq!(self.number_of_objectives(), y.len());

        unsafe {
            coco_sys::coco_evaluate_function(self.inner, x.as_ptr(), y.as_mut_ptr());
        }
    }

    pub fn final_target_hit(&self) -> bool {
        unsafe { coco_sys::coco_problem_final_target_hit(self.inner) == 1 }
    }

    pub fn dimension(&self) -> usize {
        unsafe {
            coco_sys::coco_problem_get_dimension(self.inner)
                .try_into()
                .unwrap()
        }
    }

    pub fn number_of_objectives(&self) -> usize {
        unsafe {
            coco_sys::coco_problem_get_number_of_objectives(self.inner)
                .try_into()
                .unwrap()
        }
    }

    pub fn get_ranges_of_interest(&self) -> Vec<RangeInclusive<f64>> {
        let dimension = self.dimension() as isize;
        unsafe {
            let smallest = coco_sys::coco_problem_get_smallest_values_of_interest(self.inner);
            let largest = coco_sys::coco_problem_get_largest_values_of_interest(self.inner);

            let ranges = (0..dimension)
                .into_iter()
                .map(|i| (*smallest.offset(i))..=(*largest.offset(i)))
                .collect();

            ranges
        }
    }
}

impl Drop for Problem {
    fn drop(&mut self) {
        unsafe {
            coco_sys::coco_problem_free(self.inner);
        }
    }
}

pub struct Observer {
    inner: *mut coco_observer_t,
}

impl Observer {
    pub fn new(name: &str, options: &str) -> Option<Observer> {
        let name = CString::new(name).unwrap();
        let options = CString::new(options).unwrap();

        let inner = unsafe { coco_sys::coco_observer(name.as_ptr(), options.as_ptr()) };

        if inner.is_null() {
            None
        } else {
            Some(Observer { inner })
        }
    }

    pub fn result_folder(&self) -> &str {
        unsafe {
            CStr::from_ptr(coco_sys::coco_observer_get_result_folder(self.inner))
                .to_str()
                .unwrap()
        }
    }
}

impl Drop for Observer {
    fn drop(&mut self) {
        unsafe {
            coco_sys::coco_observer_free(self.inner);
        }
    }
}
