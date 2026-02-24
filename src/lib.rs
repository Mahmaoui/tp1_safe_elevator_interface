#[derive(Clone, PartialEq, Debug)]
pub struct Elevator {
    pub state: State,
    pub floor: i32,
    pub queue: Vec<i32>
}

#[derive(Debug)]
#[derive(PartialEq, Clone)]
pub enum State {
    Idle,
    MovingUp,
    MovingDown,
    DoorsOpen
}

#[derive(Debug, PartialEq, Eq)]
pub enum ElevatorError {
    InvalidFloor(i32),
    DoorsAlreadyOpen,
    DoorsAlreadyClosed,
    CannotOpenWhileMoving,
    CannotMoveDoorsOpen,
    EmptyQueue
}

impl Elevator {
    pub fn new(floor: i32)-> Result<Self, ElevatorError> {
        if floor < 0 || floor > 5 {
           return Err(ElevatorError::InvalidFloor(floor))
        }
        Ok(Self {
            state: State::Idle,
            floor: floor,
            queue: Vec::new()
        })
    }
    pub fn call(& mut self, floor: i32)-> Result<(), ElevatorError> {
        if floor < 0 || floor > 5 {
            return Err(ElevatorError::InvalidFloor(floor));
        }
        if floor == self.floor || self.queue.contains(&floor) {
            return Ok(());
        }
        if self.floor > floor {
            self.queue.push(floor);
            if self.state == State::Idle {
                self.state = State::MovingDown;
            }
            return Ok(());
        }
        else if self.floor < floor {
            self.queue.push(floor);
            if self.state == State::Idle {
                self.state = State::MovingUp;
            }
            return Ok(());
        }
        return Ok(());
    }
    pub fn state(&self) -> State {
        self.state.clone()
    }
    pub fn step(& mut self) -> Result<(), ElevatorError>{
        if self.state == State::DoorsOpen {
            return Err(ElevatorError::CannotMoveDoorsOpen);
        }
        if self.queue.is_empty() {
            self.state = State::Idle;
            return Err(ElevatorError::EmptyQueue);
        }
        let destination: i32 = self.queue[0];
        if self.floor < destination {
            self.floor += 1;
        }
        else if self.floor > destination {
            self.floor -= 1;
        }
        if self.floor == destination {
            self.state = State::DoorsOpen;
            self.queue.remove(0);
        }
        return Ok(());
    }
    pub fn floor(&self) -> i32 {
        return self.floor;
    }
    pub fn queue(&self) -> &Vec<i32> {
        return &self.queue;
    }
    pub fn open_doors(&mut self) -> Result<(), ElevatorError>{
        if self.state == State::DoorsOpen {
            return Err(ElevatorError::DoorsAlreadyOpen);
        }
        if self.state == State::MovingUp || self.state == State::MovingDown {
            return Err(ElevatorError::CannotOpenWhileMoving);
        }
        self.state = State::DoorsOpen;
        return Ok(());
    }
    pub fn close_doors(&mut self) -> Result<(), ElevatorError> {
        if self.state != State::DoorsOpen {
            return Err(ElevatorError::DoorsAlreadyClosed);
        }
        if self.queue.is_empty() {
            self.state = State::Idle;
        }
        else {
            let next = self.queue[0];
            if next > self.floor {
                self.state = State::MovingUp;
            }
            else {
                self.state = State::MovingDown;
            }
        }
        return Ok(());
    }
    pub fn status(&self) -> Elevator {
        return Self { 
            state: self.state.clone(), 
            floor: self.floor.clone(), 
            queue: self.queue.clone()
         }
    }
}