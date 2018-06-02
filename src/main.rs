#[macro_use]
extern crate log;
extern crate rand;
extern crate web_logger;
#[macro_use]
extern crate yew;

use yew::prelude::*;

pub const HEIGHT: usize = 30;
pub const WIDTH: usize = 40;

#[derive(Debug)]
pub struct Cell {
    mined: bool,
    exposed: bool,
    flagged: bool,
    mine_count: usize,
}

pub type Board = Vec<Cell>;

fn generate_cell() -> Cell {
    Cell {
        mined: if rand::random::<f32>() <= 0.2 {
            true
        } else {
            false
        },
        exposed: false,
        flagged: false,
        mine_count: 0,
    }
}

fn generate_board() -> Board {
    let mut board = Board::new();
    for _x in 0..(WIDTH * HEIGHT) {
        board.push(generate_cell());
    }
    board
}

fn game_over(board: &Board) -> bool {
    board.iter().filter(|c| c.exposed && c.mined).count() > 0
}

fn neighbours(idx: usize) -> Vec<usize> {
    let mut neighbours = Vec::new();
    let x = idx % WIDTH;
    let y = idx / WIDTH;
    let xs1 = x.checked_sub(1);
    let xa1 = x.checked_add(1);
    let ys1 = y.checked_sub(1);
    let ya1 = y.checked_add(1);
    if xs1.is_some() && ys1.is_some() {
        neighbours.push(ys1.unwrap() * WIDTH + xs1.unwrap());
    }
    if ys1.is_some() {
        neighbours.push(ys1.unwrap() * WIDTH + x);
    }
    if xa1.is_some() && xa1.unwrap() < WIDTH && ys1.is_some() {
        neighbours.push(ys1.unwrap() * WIDTH + xa1.unwrap());
    }
    if xs1.is_some() {
        neighbours.push(y * WIDTH + xs1.unwrap());
    }
    if xa1.is_some() && xa1.unwrap() < WIDTH {
        neighbours.push(y * WIDTH + xa1.unwrap());
    }
    if xs1.is_some() && ya1.is_some() && ya1.unwrap() < HEIGHT {
        neighbours.push(ya1.unwrap() * WIDTH + xs1.unwrap());
    }
    if ya1.is_some() && ya1.unwrap() < HEIGHT {
        neighbours.push(ya1.unwrap() * WIDTH + x);
    }
    if xa1.is_some() && xa1.unwrap() < WIDTH && ya1.is_some() && ya1.unwrap() < HEIGHT {
        neighbours.push(ya1.unwrap() * WIDTH + xa1.unwrap());
    }
    neighbours
}

fn expose_cells(pos: usize, board: &mut Board) {
    fn get_count(board: &Board, indices: &Vec<usize>) -> usize {
        indices.iter().filter(|idx| board[**idx].mined).count()
    }
    fn expose_mines(board: &mut Board) {
        board.into_iter().for_each(|cell| {
            if cell.mined {
                cell.exposed = true;
            }
        });
    }

    if !board[pos].exposed && !board[pos].flagged {
        let (mined, exposed, flagged) = {
            let Cell {
                mined,
                exposed,
                flagged,
                mine_count: _,
            } = board[pos];
            (mined, exposed, flagged)
        };
        let indices = neighbours(pos);
        let count = get_count(&board, &indices);
        let check_list = if mined || exposed || flagged || count != 0 {
            Vec::new()
        } else {
            indices
        };
        if mined {
            expose_mines(board);
        } else {
            board[pos].exposed = true;
            board[pos].mine_count = count;
            for p in check_list {
                expose_cells(p, board);
            }
        }
    }
}

type Context = ();

struct Model {
    board: Board,
}

enum Msg {
    Reset,
    LeftPick(usize),
    RightPick(usize),
}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _env: &mut Env<Context, Self>) -> Self {
        Model {
            board: generate_board(),
        }
    }

    fn update(&mut self, msg: Self::Message, _env: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Reset => {
                self.board = generate_board();
                true
            }
            Msg::LeftPick(idx) => if game_over(&self.board) {
                false
            } else {
                expose_cells(idx, &mut self.board);
                true
            },
            Msg::RightPick(idx) => if game_over(&self.board) {
                false
            } else {
                if self.board[idx].exposed {
                    false
                } else {
                    self.board[idx].flagged = !self.board[idx].flagged;
                    true
                }
            },
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html!{
            <div class="game",>
                <div class="face",></div>
                <div class="board",>
                {
                   for self.board.iter().enumerate().map(view_cell)
                }
                </div>
                <div class="controls",>
                    <button onclick=|_| Msg::Reset,>{ "Reset" }</button>
                </div>
            </div>
        }
    }
}

fn view_cell((idx, cell): (usize, &Cell)) -> Html<Context, Model> {
    html!{
        <div class=("cell", if cell.exposed { "exposed" } else { "" }, if cell.exposed && cell.mined { "mined" } else { "" }, if cell.flagged { "flagged" } else {""}), onclick=|_| Msg::LeftPick(idx), oncontextmenu=|e| rightclick(idx, e),>
        {
            if cell.exposed && cell.mine_count > 0 {
                cell.mine_count.to_string()
            } else {
                "".to_string()
            }
        }
        </div>
    }
}

fn rightclick(idx: usize, event: ContextMenuEvent) -> Msg {
    event.prevent_default();
    Msg::RightPick(idx)
}

fn main() {
    web_logger::init();
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
    trace!("Started");
}
