pub fn assignment1(){
    let bits_len = 1200;
    let sampling_frequency = 15000;
    let (x, y) = assignment1_makedata(bits_len, sampling_frequency);

    use gnuplot::{Figure, Color, AxesCommon};

    let mut fg = Figure::new();
    fg.axes2d()
    .set_x_label("Time [micro sec]", &[])
    .set_y_label("Bits", &[])
    .lines(&x, &y, &[Color("black")]);
    fg.show().unwrap();
}

fn assignment1_makedata(bits_len: usize, sampling_frequency: usize) -> (Vec<f64>, Vec<f64>) {
    use super::generate_data::generate_random_bits;
    let x = vec![0;bits_len].iter()
                .enumerate()
                .map(|(i, _)| i as f64 / (bits_len*sampling_frequency) as f64 * 1e6)
                .collect::<Vec<f64>>();
    let y = generate_random_bits(bits_len);
                //.iter()
                // .map(|&e| vec![e; sampling_frequency])
                // .flatten()
                // .collect::<Vec<u8>>();
    (x,y)
}

pub fn assignment2() {
    let bits_len = 1200;
    let sampling_frequency = 15000;
    let (x, y) = assignment2_makedata(bits_len, sampling_frequency);

    use gnuplot::{Figure, Color, AxesCommon};

    let mut fg = Figure::new();
    fg.axes2d()
    .set_x_label("Time [micro sec]", &[])
    .set_y_label("Noise", &[])
    .lines(&x, &y, &[Color("black")]);
    fg.show().unwrap();
}

fn assignment2_makedata(bits_len: usize, sampling_frequency: usize) -> (Vec<f64>, Vec<f64>) {
    use super::generate_data::generate_noises;
    let x = vec![0;bits_len].iter()
                .enumerate()
                .map(|(i, _)| i as f64 / (bits_len*sampling_frequency) as f64 * 1e6)
                .collect::<Vec<f64>>();
    let y = generate_noises(bits_len);
    (x,y)
}

pub fn assignment3() {

    let (hi, hq, t) = assignment3_makedata();

    use gnuplot::{Figure, Color, AxesCommon};

    let mut fg = Figure::new();
    fg.axes2d()
    .set_title("I-phase", &[])
    .set_x_label("Delay [micro sec]", &[])
    // .set_y_label("Bits", &[])
    .set_y_label("Amplitude", &[])
    .boxes(t.iter(), hi.iter(), &[Color("black")]);
    fg.show().unwrap();

    let mut fg = Figure::new();
    fg.axes2d()
    .set_title("Q-phase", &[])
    .set_x_label("Delay [micro sec]", &[])
    // .set_y_label("Bits", &[])
    .set_y_label("Amplitude", &[])
    .boxes(t.iter(), hi.iter(), &[Color("black")]);
    fg.show().unwrap();

}

fn assignment3_makedata() -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = vec![1, 5, 6, 7, 8, 15, 18, 21, 24, 29, 39, 42, 43, 45, 47, 50, 90];
    let p = vec![-18.8, -2.7, -7.3, -15.3, -12.9, -21.3, -12.0, -16.4, -14.9, -16.7, -18.1, -21.6, -23.6, -23.7, -24.3, -22.0, -25.3, -35.1];
    let p = p.iter().map(|e| 10.0f64.powf(e/10.0)).collect::<Vec<f64>>();
    let tau = (0..144).map(|e| e as f64 / 2048.0 / 14.0 / 1e3 * 1e6).collect::<Vec<f64>>();
    let mut hi = vec![0.0; 144];
    let mut hq = vec![0.0; 144];

    use super::generate_data::generate_noises;
    let gi = generate_noises(17);
    let gq = generate_noises(17);

    for (i, &e) in n.iter().enumerate() {
        hi[e] = (p[i]/2.0f64).sqrt() * gi[i];
        // println!("{}", hi[e]);
        hq[e] = (p[i]/2.0f64).sqrt() * gq[i];
    }
    // println!("{:?}", hi);

    (hi, hq, tau)
}