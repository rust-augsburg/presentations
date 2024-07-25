pub use candle::{DType, Device, Module, Tensor};
use candle_nn::{loss, ops, Linear, Optimizer, VarBuilder, VarMap};
use plotters::{
    chart::ChartBuilder,
    prelude::{IntoDrawingArea, PathElement, SVGBackend},
    series::LineSeries,
    style::{Color, BLACK, RED, WHITE},
};
use rand::{prelude::SliceRandom, rngs::SmallRng, SeedableRng};

const IN_DIM: usize = 1;
const OUT_DIM: usize = 1;
const LAYER1_OUT_SIZE: usize = 2 * 64;
const LAYER2_OUT_SIZE: usize = 2 * 32;
const LEARNING_RATE: f64 = 0.005;

#[derive(Clone)]
pub struct Dataset {
    pub train_input: Vec<Tensor>,
    pub train_results: Vec<Tensor>,
    pub test_input: Tensor,
    pub test_results: Tensor,
}

pub struct Model {
    ln1: Linear,
    ln2: Linear,
    ln3: Linear,
}

impl Model {
    fn new(vs: VarBuilder) -> candle::Result<Self> {
        let ln1 = candle_nn::linear(IN_DIM, LAYER1_OUT_SIZE, vs.pp("ln1"))?;
        let ln2 = candle_nn::linear(LAYER1_OUT_SIZE, LAYER2_OUT_SIZE, vs.pp("ln2"))?;
        let ln3 = candle_nn::linear(LAYER2_OUT_SIZE, OUT_DIM, vs.pp("ln3"))?;

        Ok(Self { ln1, ln2, ln3 })
    }

    pub fn forward(&self, xs: &Tensor) -> candle::Result<Tensor> {
        // dbg!(xs.shape());
        let xs = self.ln1.forward(xs)?;
        // dbg!(xs.shape());
        // let xs = xs.relu()?;
        let xs = ops::sigmoid(&xs)?;
        let xs = self.ln2.forward(&xs)?;
        let xs = ops::sigmoid(&xs)?;
        // let xs = xs.relu()?;
        self.ln3.forward(&xs)
    }
}

pub fn train(m: Dataset, epochs: usize) -> anyhow::Result<Model> {
    let dev = &Device::Cpu;

    let test_input = m.test_input.to_device(dev)?;
    let test_results = m.test_results.to_device(dev)?;

    let varmap = VarMap::new();
    let vs = VarBuilder::from_varmap(&varmap, DType::F32, dev);
    let model = Model::new(vs.clone())?;
    let mut optim = candle_nn::AdamW::new_lr(varmap.all_vars(), LEARNING_RATE)?;

    let mut prev_loss = f32::MAX;
    let mut cooldown = 10_u8;
    for epoch in 0..epochs {
        let mut losses = Vec::new();
        for (input, results) in m.train_input.iter().zip(m.train_results.iter()) {
            let train_input = input.to_device(dev)?;
            let train_results = results.to_device(dev)?;

            let pred = model.forward(&train_input)?;
            let loss = loss::mse(&pred, &train_results)?;
            optim.backward_step(&loss)?;

            let total_loss = loss.sum_all()?.to_scalar::<f32>()?;
            losses.push(total_loss);
        }

        let loss = losses.iter().sum::<f32>() / losses.len() as f32;
        if cooldown == 0 && prev_loss < loss * 1.001 {
            optim.set_learning_rate(optim.learning_rate() * 0.7);
            println!("Reduced learning rate to {}", optim.learning_rate());
            cooldown = 12;
        } else {
            cooldown = cooldown.saturating_sub(1);
            prev_loss = loss;
        }

        let test_pred = model.forward(&test_input)?;
        let test_accuracy = loss::mse(&test_pred, &test_results)?
            .sum_all()?
            .to_scalar::<f32>()?;
        let final_accuracy = 100. * test_accuracy;
        println!(
            "Epoch: {epoch:3} Train loss: {:8.5} Test accuracy: {:5.4}%",
            loss, final_accuracy
        );
    }
    Ok(model)
}

fn main() -> anyhow::Result<()> {
    let mut x: Vec<f32> = (0..400).map(|v| v as f32 / 100.0 * 7.0).collect();
    let mut y: Vec<f32> = x.iter().copied().map(f32::sin).collect();

    x.shuffle(&mut SmallRng::from_seed([1; 32]));
    y.shuffle(&mut SmallRng::from_seed([1; 32]));

    fn tensors_from_slice(slice: &[f32]) -> candle::Result<Vec<Tensor>> {
        slice
            .chunks(10)
            .map(|chunk| {
                let shape = chunk.len();
                Tensor::from_slice(chunk, (shape, 1), &Device::Cpu)
            })
            .collect::<Result<_, _>>()
    }

    let x = tensors_from_slice(&x)?;
    let y = tensors_from_slice(&y)?;

    let test_x: Vec<f32> = (00..500).map(|v| v as f32 / 100.0 * 7.0).collect();
    let test_y: Vec<f32> = test_x.iter().copied().map(f32::sin).collect();

    fn tensor_from_vec(vec: Vec<f32>) -> candle::Result<Tensor> {
        let len = vec.len();
        Tensor::from_vec(vec, (len, 1), &Device::Cpu)
    }

    let test_x = tensor_from_vec(test_x)?;
    let test_y = tensor_from_vec(test_y)?;

    let m = Dataset {
        train_input: x,
        train_results: y,
        test_input: test_x.clone(),
        test_results: test_y.clone(),
    };

    let model = train(m, 40)?;

    let test_prediction = model.forward(&test_x)?;
    let test_prediction = test_prediction.to_vec2::<f32>()?;
    let test_y = test_y.to_vec2::<f32>()?;

    let test_prediction: Vec<f32> = test_prediction.into_iter().flatten().collect();
    let test_y: Vec<f32> = test_y.into_iter().flatten().collect();

    plot(&test_prediction, "prediction.svg")?;
    plot(&test_y, "expected.svg")?;

    Ok(())
}

// Utility method
fn plot(data: &[f32], path: &str) -> anyhow::Result<()> {
    let max = data.iter().copied().reduce(f32::max).unwrap_or_default();
    let min = data.iter().copied().reduce(f32::min).unwrap_or_default();

    let root = SVGBackend::new(path, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        // .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-0f32..data.len() as f32, min..max)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            data.iter()
                .enumerate()
                .map(|(idx, value)| (idx as f32, *value)),
            RED,
        ))?
        // .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
