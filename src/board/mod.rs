mod Board{
    
    struct Board{
        width:usize,
        height:usize,
        background_color:String,
        space_width:usize,
        space_height:usize,
        space_border:usize,
        branch_width:usize,
        branch_color:String,
        spaces:Vec<BoardSpace>,
        edges:Vec<BoardEdge>
    }


    impl Board{
        fn new(
        width:usize,
        height:usize,
        background_color:String,
        space_width:usize,
        space_height:usize,
        space_border:usize,
        branch_width:usize,
        branch_color:String
            )->Board{
            Board {
                width:width,
                height: height,
                background_color:background_color,
                space_width:space_width,
                space_height:space_height,
                space_border:space_border,
                branch_width:branch_width,
                branch_color:branch_color,
                spaces: Vec<BoardSpace>::new(),
                edges: Vec<BoardEdge>::new() 
            } 
        }
        
        fn push_space(&mut self, &space:BoardSpace){
            &mut self.spaces.push(space);
        }

        fn push_edge(&mut self, &edge:BoardEdge){
            &mut self.edges.push(edge);
        }
        
        pub fn get_spaces(&self){
            &self.spaces
        }

        pub fn get_edges(&self){
            &self.edges
        }
        pub fn to_string(&self){
            println!("width:{self.width}");
        }
    }
}

struct BoardSpace{
    id:String,
    pos_x:usize,
    pos_y:usize,
    connected:Vec<BoardSpace>,
    edges:Vec<BoardEdge>,
    next:BoardSpace,
    shortcut:Option<BoardSpace>,
    is_start:bool,
    is_end:bool
}

struct BoardEdge{
    id:String,
    nodes:[BoardSpace;2]
}

