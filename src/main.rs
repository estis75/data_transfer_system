mod dft;
mod complex;
mod generate_data;
// mod week1;

fn main() {
    assignment2();
}

fn assignment1(){
    // powerをあとでけいさんするようにする.

    let bits_len = 2400;
    let sampling_frequency = 15000;
    let senddata_length = 2048;
    let qam = 4;

    let x = assignment1_makedata(bits_len, senddata_length, qam);

    let (hi, hq): (Vec<f64>, Vec<f64>) = x.iter().map(|&c| (c.real_part(), c.imaginary_part())).unzip();

    let t = vec![0;x.len()].iter()
                .enumerate()
                .map(|(i, _)| i as f64 / (senddata_length*sampling_frequency) as f64 * 1e6)
                .collect::<Vec<f64>>();

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

fn assignment1_makedata(bits_len: usize, senddata_length: usize, qam: usize) -> Vec<Complex> {

    use generate_data::{generate_random_bits, qam_mapping};
    let original_data = generate_random_bits(bits_len);
    let symbols = qam_mapping(original_data, qam);

    let mut vec = Vec::with_capacity(senddata_length);
    for _ in 0..424{ vec.push(Complex::zero()); }
    for &e in symbols.iter() {
        vec.push(e.mulr((senddata_length as f64 / symbols.len() as f64).sqrt()));
    }
    for _ in vec.len()..senddata_length { vec.push(Complex::zero()); }

    let x = dft::ifft(vec);
    let x = generate_data::cpa(x, 144);
    dbg!(x.len());
    x
}

fn assignment2(){
    let bits_len = 1200;
    let sampling_frequency = 15000;
    let senddata_length = 2048;
    let qam = 4;

    let recv = assignment2_makedata(bits_len, senddata_length, qam);
    dbg!(recv.len());

    let t = vec![0;recv.len()].iter()
                .enumerate()
                .map(|(i, _)| i as f64 / (senddata_length*sampling_frequency) as f64 * 1e6)
                .collect::<Vec<f64>>();
    let (hi, hq): (Vec<f64>, Vec<f64>) = recv.iter().map(|&h| (h.real_part(), h.imaginary_part())).unzip();

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

fn assignment2_makedata(bits_len: usize, senddata_length: usize, qam: usize) -> Vec<Complex> {
    let mut recv = assignment1_makedata(bits_len, senddata_length, qam);
    let recv_len = recv.len();

    let n = vec![1, 5, 6, 7, 8, 15, 18, 21, 24, 29, 39, 42, 43, 45, 47, 50, 90];
    let p = vec![-18.8, -2.7, -7.3, -15.3, -12.9, -21.3, -12.0, -16.4, -14.9, -16.7, -18.1, -21.6, -23.6, -23.7, -24.3, -22.0, -25.3, -35.1];
    let p = p.iter().map(|e| 10.0f64.powf(e/10.0)).collect::<Vec<f64>>();
    let mut h = vec![Complex::zero(); 144];

    use generate_data::generate_noises;
    let gi = generate_noises(17);
    let gq = generate_noises(17);
    let g = gi.iter().zip(gq.iter()).map(|(&l, &r)| cp!((l) + (r)i)).collect::<Vec<Complex>>();

    for (i, &e) in n.iter().enumerate() {
        h[e] = g[i].mulr((p[i]/2.0f64).sqrt());
    }
    dbg!(&h);

    let mut v = vec![Complex::zero(); 143];
    v.append(&mut recv);

    let mut ret = vec![Complex::zero(); recv_len];
    for i in 0..ret.len() {
        for j in 0..144 {
            ret[i] += v[i+j].dot(h[144-j-1]);
        }
    }
    dbg!(v[0], ret[0]);
    ret
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