use vega_lite_3::*;

use rulinalg::matrix::Matrix;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A new matrix with 4 rows and 2 columns.
    let values = Matrix::new(4, 2, (1..9).collect::<Vec<usize>>());
    // the chart
    let chart = VegaliteBuilder::default()
        .title("Random points")
        .data(values)
        .mark(Mark::Point)
        .encoding(
            EncodingBuilder::default()
                .x(XClassBuilder::default()
                    .field("0")
                    .def_type(StandardType::Quantitative)
                    .build()?)
                .y(YClassBuilder::default()
                    .field("1")
                    .def_type(StandardType::Quantitative)
                    .build()?)
                .build()?,
        )
        .build()?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
