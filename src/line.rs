pub mod line{
    #[derive(Eq, PartialEq)]
    pub enum LineAssembleMode{
        Loop,
        Strip,
    }
   pub fn line_assemble(mode: LineAssembleMode,vertices:u32)->Vec<u32>{
        let mut indices=vec![];
        let mut last_vertex=0;
        let mut counter=0;
        for _i in 0..vertices{
            match mode{
                //OK
                LineAssembleMode::Loop => {
                    last_vertex=counter;
                    counter+=1;
                    indices.push(last_vertex);
                    indices.push(counter);
                    if vertices ==counter{
                        indices.push(counter);
                        indices.push(0);
                    }
                },
                //Ok
                LineAssembleMode::Strip => {
                    last_vertex=counter;
                    counter+=1;
                    indices.push(last_vertex);
                    indices.push(counter);
                },
            }
        }
        indices
    }
}