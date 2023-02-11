use json;


pub struct Board{
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
            width,
            height,
            background_color,
            space_width,
            space_height,
            space_border,
            branch_width,
            branch_color,
            spaces: Vec::new(),
            edges: Vec::new() 
        } 
    }
    
    fn push_space(&mut self, space:BoardSpace){
        &mut self.spaces.push(space);
    }

    fn push_edge(&mut self, edge:BoardEdge){
        &mut self.edges.push(edge);
    }
    
    pub fn get_spaces(&self)->&Vec<BoardSpace>{
        &self.spaces
    }

    pub fn get_edges(&self)->&Vec<BoardEdge>{
        &self.edges
    }
    pub fn get_width(&self)->usize{
        self.width
    }
    pub fn from_json_string(json_string:String)->Self{
        let mut parsed = json::parse(&*json_string).unwrap();
        println!("{}",parsed);
        let mut board = Self::new(
            parsed["board"]["width"].as_usize().unwrap(), 
            parsed["board"]["height"].as_usize().unwrap(),
            parsed["board"]["background_color"].take_string().unwrap(),
            parsed["board"]["space_width"].as_usize().unwrap(),
            parsed["board"]["space_height"].as_usize().unwrap(),
            parsed["board"]["space_border"].as_usize().unwrap(),
            parsed["board"]["branch_width"].as_usize().unwrap(),
            parsed["board"]["branch_color"].take_string().unwrap()
        );

        println!("{}",board.get_width());
        let members = parsed["spaces"].members_mut();
        
        let mut spaces:Vec<BoardSpace> = Vec::new();
        for space in members{
            println!("space before create");
            let board_space = BoardSpace::new(
                space["id"].take_string().unwrap(),
                space["pos_x"].as_usize().unwrap(),
                space["pos_y"].as_usize().unwrap(),
                space["is_start"].as_bool().unwrap(),
                space["is_last"].as_bool().unwrap()
            );
            spaces.push(board_space);
        }
//        for item in board.spaces{
//        let matched = board.spaces.iter().find(|&&x|x.id==item.id).unwrap_or(None);
//        if (matched!=None){
//            item.connected.push(&matched);
//        }

        
//        }
        for item in &spaces{
            println!("{}", item.id);
        }
    

        board
    }
}


struct BoardSpace{
    id:String,
    pos_x:usize,
    pos_y:usize,
    connected:Vec<BoardSpace>,
    edges:Vec<BoardEdge>,
    next:Option<Box<BoardSpace>>,
    shortcut:Option<Box<BoardSpace>>,
    is_start:bool,
    is_end:bool
}

impl BoardSpace{
    fn new(
        id:String,
        pos_x:usize,
        pos_y:usize,
        is_start:bool,
        is_end:bool
        ) -> Self{
        BoardSpace{
            id,
            pos_x,
            pos_y,
            connected:Vec::new(),
            edges:Vec::new(),
            next:None,
            shortcut:None,
            is_start,
            is_end
        }

    }

    fn is_valid(&self) -> bool{

        if (self.next.is_none()){
            if (!self.is_end){
            return false;
            }
        }
        true
    }
    
    fn get_edges(&self)-> &Vec<BoardEdge>{
        &self.edges
    }
}


struct BoardEdge{ 
    id:String, 
    nodes:Vec<BoardSpace> 
}
impl BoardEdge{
    fn new(id:String, nodes:Vec<BoardSpace>) -> Self{
        BoardEdge{
            id,
            nodes:Vec::new()
        }
    }
    fn is_valid(&self)->bool   
    {
        return true;
        
    }

    
}

