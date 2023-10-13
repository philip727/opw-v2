pub trait ValueMap2D<TValue>
where
    TValue: PartialEq + Sized + Clone,
{
    fn new(size: (usize, usize)) -> Self;
    fn get_size(&self) -> (usize, usize);
    fn get_points(&self) -> &[TValue];
    fn mut_points(&mut self) -> &mut Vec<TValue>;

    fn set_value(&mut self, x: usize, y: usize, value: TValue) {
        let (width, height) = self.get_size();

        if x < width && y < height {
            self.mut_points()[x + y * width] = value;
        }
    }

    fn get_value(&self, x: usize, y: usize) -> Option<TValue> {
        let (width, height) = self.get_size();

        if x < width && y < height {
            return Some(self.get_points()[x + y * width].clone());
        }

        None
    }
}
