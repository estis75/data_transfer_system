mod generate_data;

fn assignment1(){
    let bits_len = 1200;
    let sampling_frequency = 15000;
    let t = vec![0;bits_len].iter()
                .enumerate()
                .map(|(i, _)| i as f64 / (bits_len*sampling_frequency) as f64 * 1e6)
                .collect::<Vec<f64>>();
    let (hi, hq): (Vec<f64>, Vec<f64>) = generate_data::generate_random_n_qam_signal(1200, 4).iter().map(|&h| (h.real_part(), h.imaginary_part())).unzip();
    use gnuplot::{Figure, Color, AxesCommon};

    let mut fg = Figure::new();
    fg.axes2d()
    .set_title("I-phase", &[])
    .set_x_label("Time [micro sec]", &[])
    // .set_y_label("Bits", &[])
    .set_y_label("Amplitude", &[])
    .boxes(t.iter(), hi.iter(), &[Color("blue")]);
    fg.show().unwrap();

    let mut fg = Figure::new();
    fg.axes2d()
    .set_title("Q-phase", &[])
    .set_x_label("Time [micro sec]", &[])
    // .set_y_label("Bits", &[])
    .set_y_label("Amplitude", &[])
    .boxes(t.iter(), hq.iter(), &[Color("red")]);
    fg.show().unwrap();

}

fn assignment2(){
    let bits_len = 1200;
    let sampling_frequency = 15000;
    let t = vec![0;bits_len].iter()
                .enumerate()
                .map(|(i, _)| i as f64 / (bits_len*sampling_frequency) as f64 * 1e6)
                .collect::<Vec<f64>>();
    let (hi, hq): (Vec<f64>, Vec<f64>) = generate_data::generate_random_n_qam_signal(1200, 256).iter().map(|&h| (h.real_part(), h.imaginary_part())).unzip();

    use gnuplot::{Figure, Color, AxesCommon};

    let mut fg = Figure::new();
    fg.axes2d()
    .set_title("I-phase", &[])
    .set_x_label("Time [micro sec]", &[])
    // .set_y_label("Bits", &[])
    .set_y_label("Amplitude", &[])
    .boxes(t.iter(), hi.iter(), &[Color("blue")]);
    fg.show().unwrap();

    let mut fg = Figure::new();
    fg.axes2d()
    .set_title("Q-phase", &[])
    .set_x_label("Time [micro sec]", &[])
    // .set_y_label("Bits", &[])
    .set_y_label("Amplitude", &[])
    .boxes(t.iter(), hq.iter(), &[Color("red")]);
    fg.show().unwrap();

}

fn assignment3(){
    let bits_len = 1200;
    let sampling_frequency = 15000;
    let t = vec![0;bits_len].iter()
                .enumerate()
                .map(|(i, _)| i as f64 / (bits_len*sampling_frequency) as f64 * 1e6)
                .collect::<Vec<f64>>();
    let h: Vec<Complex> = assignment3_makedata(bits_len, 256);
    let hi = h.iter().map(|&c| c.real_part()).collect::<Vec<f64>>();
    let hq = h.iter().map(|&c| c.imaginary_part()).collect::<Vec<f64>>();

    use gnuplot::{Figure, Color, AxesCommon};

    let mut fg = Figure::new();
    fg.axes2d()
    .set_title("I-phase", &[])
    .set_x_label("Time [micro sec]", &[])
    // .set_y_label("Bits", &[])
    .set_y_label("Amplitude", &[])
    .boxes(t.iter(), hi.iter(), &[Color("blue")]);
    fg.show().unwrap();

    let mut fg = Figure::new();
    fg.axes2d()
    .set_title("Q-phase", &[])
    .set_x_label("Time [micro sec]", &[])
    // .set_y_label("Bits", &[])
    .set_y_label("Amplitude", &[])
    .boxes(t.iter(), hq.iter(), &[Color("red")]);
    fg.show().unwrap();
}

use complex::Complex;
fn assignment3_makedata(length: usize, psk: usize) -> Vec<Complex> {
    let qam = generate_data::generate_random_n_qam_signal(length, psk);

    use generate_data::generate_noises;
    let gi = generate_noises(length);
    let gq = generate_noises(length);
    let noise = gi.iter().zip(gq.iter()).map(|(&l,&r)| cp!((l) + (r)i)).collect::<Vec<Complex>>();

    let pz = 0.5f64;
    qam.iter().zip(noise.iter()).map(|(&h, &g)| h + g.mulr((pz/2.0).sqrt())).collect::<Vec<Complex>>()
}