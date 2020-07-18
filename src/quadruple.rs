pub mod quadruple{
    #[derive(Eq, PartialEq)]
    pub enum QuadAssembleMode {
        Separate,
        Strip
    }

    pub fn quads_assemble(mode: QuadAssembleMode, mut vertices:u32) ->Vec<u32>{
        let mut indices=vec![];
        let mut counter: u32=0;
        // vertices count
        let mut vertex_a: u32=0;
        // index a
        let mut vertex_b: u32=0;
        // index b
        let mut vertex_c:u32=0;
        //index c
        let mut target: bool=true;
        let emit_last={if vertices%2==1 {
            false
        }else{
            true
        }
        };
        if mode==QuadAssembleMode::Strip{
            vertices+=1;
        }
        // if vertices odd we must block last triangle
        if !emit_last{
            vertices-=1;
        }
        for _i in  0..vertices{
            match mode{
                QuadAssembleMode::Strip => {
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
                }
                //OK
                QuadAssembleMode::Separate => {
                    counter+=1;
                    if counter%4==1{
                        vertex_a=counter;
                    }
                    if counter%4==2{
                        vertex_b=counter;
                    }
                    if counter%4==3 {
                        vertex_c=counter;
                    }
                    if counter%4==0{
                        indices.push(vertex_a-1);
                        indices.push(vertex_b-1);
                        indices.push(vertex_c-1);
                        indices.push(vertex_c-1);
                        indices.push(vertex_b-1);
                        indices.push(counter-1);
                    }

                }
            }
        }
        indices
    }
}