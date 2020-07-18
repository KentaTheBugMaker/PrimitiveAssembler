pub mod triangle {
    #[derive(Eq, PartialEq)]
    pub enum TriangleAssembleMode {
        Fan,
        Strip,
        Separate,
    }

        //if triangle is constructed emit face
        pub fn triangle_assemble(mode: TriangleAssembleMode, mut vertices:u32) ->Vec<u32>{
            let mut indices=vec![];
            let mut counter=0;
            let mut vertex_b=0;
            let mut vertex_a=0;
            let mut target=true;
            //TriangleStrip Specific Code
            if mode==TriangleAssembleMode::Strip{
              vertices+=1
            }
            for _i in 0..vertices{
                match mode {
                    //Ok
                    TriangleAssembleMode::Fan => {
                        if counter >= 1 {
                            vertex_b = counter;
                        }
                        counter+=1;
                        if counter > 2 {
                            indices.push(vertex_a);
                            indices.push(vertex_b-1);
                            indices.push(counter-1);
                        }
                    },
                    //OK
                    TriangleAssembleMode::Strip => {
                        // target is a
                        if target{
                            vertex_a=counter
                        }else{
                            vertex_b=counter
                        }
                        target=!target;//toggle target
                        counter+=1;
                        if counter>3{
                            indices.push(vertex_a-2);
                            indices.push(vertex_b-2);
                            indices.push(counter-2);
                        }

                    },
                    //OK
                    TriangleAssembleMode::Separate => {
                       counter+=1;
                        if counter%3==1{
                            vertex_a=counter;
                        }
                        if counter%3==2{
                            vertex_b=counter;
                        }
                        if counter%3==0{
                            indices.push(vertex_a-1);
                            indices.push(vertex_b-1);
                            indices.push(counter-1);
                        }
                    },
                }
            }
            indices
        }
}