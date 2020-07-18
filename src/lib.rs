mod line;
mod quadruple;
mod triangle;
#[cfg(test)]
mod tests {
    use crate::triangle::triangle::{TriangleAssembleMode, triangle_assemble};
    use crate::quadruple::quadruple::{quads_assemble, QuadAssembleMode};
    use crate::line::line::{line_assemble, LineAssembleMode};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let vertices=11;
        let face= triangle_assemble(TriangleAssembleMode::Strip, vertices);
        let quad_face=quads_assemble(QuadAssembleMode::Separate,vertices);
        let line=line_assemble(LineAssembleMode::Loop,vertices);
        println!("{:#?},{}",face,face.len());
        println!("{:#?},{}",quad_face,quad_face.len());
    }
}
