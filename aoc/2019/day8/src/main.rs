struct SpaceImage {
    height: usize,
    width: usize,
    data: Vec<u8>,
}

impl SpaceImage {
    fn new(width: usize, height: usize, data: &str) -> Self {
        Self {
            height,
            width,
            data: data.chars()
                      .map(|c| c.to_digit(10).unwrap() as u8)
                      .collect(),
        }
    }

    fn layer(&self, n: usize) -> Layer {
        let per_layer = self.pixels_per_layer();
        let start = per_layer * n;
        let end = start + per_layer;
        assert!(end <= self.data.len());
        Layer {
            data: &self.data[start..end],
        }
    }

    fn num_layers(&self) -> usize {
        self.data.len() / self.pixels_per_layer()
    }

    fn pixels_per_layer(&self) -> usize {
        self.height * self.width
    }

    fn iter(&self) -> Iter {
        Iter {
            image: &self,
            index: 0,
        }
    }

    fn render(&self) {
        const BLACK: u8 = 0;
        const WHITE: u8 = 1;
        const TRANSPARENT: u8 = 2;

        let mut final_image = vec![TRANSPARENT; self.pixels_per_layer()];

        for layer in self.iter() {
            for px in 0..self.pixels_per_layer() {
                if final_image[px] != TRANSPARENT {
                    continue;
                }
                final_image[px] = layer.data[px];
            }
        }

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = (row * self.width) + col;
                if final_image[idx] == BLACK {
                    print!(" ");
                } else {
                    print!("X");
                }
            }
            print!("\n");
        }
    }
}

#[derive(Debug)]
struct Layer<'a> {
    data: &'a [u8],
}

struct Iter<'a> {
    image: &'a SpaceImage,
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Layer<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.image.num_layers() {
            None
        } else {
            let next_layer = self.image.layer(self.index);
            self.index += 1;
            Some(next_layer)
        }
    }
}


fn main() {
    let image_data = std::fs::read_to_string("input.txt").expect("did not get file");
    let image = SpaceImage::new(25, 6, image_data.trim());

    let mut fewest_zeros = usize::max_value();
    let mut ans = 0;

    for layer in image.iter() {
        let zero_count = layer.data.iter().filter(|pixel| **pixel == 0).count();
        if  zero_count < fewest_zeros {
            fewest_zeros = zero_count;
            let ones_count = layer.data.iter().filter(|pixel| **pixel == 1).count();
            let twos_count = layer.data.iter().filter(|pixel| **pixel == 2).count();
            ans = ones_count * twos_count;
        }
    }

    println!("part one! {:?}", ans);
    image.render();
}
