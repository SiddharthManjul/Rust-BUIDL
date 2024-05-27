#[derive(Debug)]
#[allow(dead_code)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    #[allow(dead_code)]
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    #[allow(dead_code)]
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    #[allow(dead_code)]
    fn update_average(&mut self) {
        if self.list.is_empty() {
            self.average = 0.0;
        } else {
            let sum: i32 = self.list.iter().sum();
            self.average = sum as f64 / self.list.len() as f64;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_and_average() {
        let mut collection = AveragedCollection {
            list: vec![],
            average: 0.0,
        };

        collection.add(1);
        dbg!(collection.average);
        assert_eq!(collection.average, 1.0);

        collection.add(2);
        dbg!(collection.average);
        assert_eq!(collection.average, 1.5);

        collection.add(3);
        dbg!(collection.average);
        assert_eq!(collection.average, 2.0);
    }

    #[test]
    fn test_remove() {
        let mut collection = AveragedCollection {
            list: vec![1, 2, 3],
            average: 2.0,
        };

        let removed_value = collection.remove();
        dbg!(collection.average);
        assert_eq!(removed_value, Some(3));
        assert_eq!(collection.average, 1.5);

        let removed_value = collection.remove();
        dbg!(collection.average);
        assert_eq!(removed_value, Some(2));
        assert_eq!(collection.average, 1.0);

        let removed_value = collection.remove();
        dbg!(collection.average);
        assert_eq!(removed_value, Some(1));
        assert_eq!(collection.average, 0.0);

        let removed_value = collection.remove();
        dbg!(collection.average);
        assert_eq!(removed_value, None);
        assert_eq!(collection.average, 0.0);
    }
}
