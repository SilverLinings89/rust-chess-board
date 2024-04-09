use yew::html::Properties;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, PartialEq)]
enum SquareState {
    Empty,
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,
    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,
}


#[derive(Debug, Clone, PartialEq)]
struct Square {
    name: String,
    color: Color,
    rank: u8,
    file: char,
}

impl Square {
    fn new(name: &str, color: Color, rank: &u8, file: &char) -> Square {
        Square {
            name: name.to_string(),
            color,
            rank: *rank,
            file: *file,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Piece {
    Empty,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

struct ChessBoard {
    board: [[Piece; 8]; 8],
}

impl ChessBoard {
    fn new() -> ChessBoard {
        ChessBoard {
            board: [
                [Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen, Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook],
                [Piece::Pawn; 8],
                [Piece::Empty; 8],
                [Piece::Empty; 8],
                [Piece::Empty; 8],
                [Piece::Empty; 8],
                [Piece::Pawn; 8],
                [Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen, Piece::King, Piece::Bishop, Piece::Knight, Piece::Rook],
            ],
        }
    }

    fn get_initial_position() -> Vec<SquareState> {
        vec![
            SquareState::WhiteRook,
            SquareState::WhiteKnight,
            SquareState::WhiteBishop,
            SquareState::WhiteKing,
            SquareState::WhiteQueen,
            SquareState::WhiteBishop,
            SquareState::WhiteKnight,
            SquareState::WhiteRook,
            SquareState::WhitePawn,
            SquareState::WhitePawn,
            SquareState::WhitePawn,
            SquareState::WhitePawn,
            SquareState::WhitePawn,
            SquareState::WhitePawn,
            SquareState::WhitePawn,
            SquareState::WhitePawn,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::Empty,
            SquareState::BlackPawn,
            SquareState::BlackPawn,
            SquareState::BlackPawn,
            SquareState::BlackPawn,
            SquareState::BlackPawn,
            SquareState::BlackPawn,
            SquareState::BlackPawn,
            SquareState::BlackPawn,
            SquareState::BlackRook,
            SquareState::BlackKnight,
            SquareState::BlackBishop,
            SquareState::BlackKing,
            SquareState::BlackQueen,
            SquareState::BlackBishop,
            SquareState::BlackKnight,
            SquareState::BlackRook,
        ]
    }

    fn get_base_board() -> Vec<Square>{
        let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let ranks: [u8; 8] = [8, 7, 6, 5, 4, 3, 2, 1];

        let mut squares = Vec::new();

        for rank in &ranks {
                for file in &files {
                let name = format!("{}{}", file, rank);
                let color = if (*file as u8 + *rank as u8) % 2 == 1 {
                    Color::White
                } else {
                    Color::Black
                };
                squares.push(Square::new(&name, color, rank, file));
            }
        }
        return squares;
    }

}


fn rank_and_file_to_index(rank: u8, file: char) -> usize {
    let rank = 8 - rank;
    let file = file as u8 - 'a' as u8;
    (rank * 8 + file) as usize
}



#[derive(PartialEq, Properties, Clone)]
struct PieceProps {
    state: SquareState,
}


#[derive(PartialEq, Properties, Clone)]
struct SquareProps {
    square: Square,
    state: SquareState,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &Props) -> Html {
    let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.html.clone());

    Html::VRef(div.into())
}

#[function_component(BoardPiece)]
fn board_piece(piece: &PieceProps) -> Html {
    html! {
        <div >
            {match piece.state {
                SquareState::WhiteKing => html! {<SafeHtml html={include_str!("./pieces/KW.svg")} />},
                SquareState::WhiteQueen => html! {<SafeHtml html={include_str!("./pieces/QW.svg")} />},
                SquareState::WhiteRook => html! {<SafeHtml html={include_str!("./pieces/RW.svg")} />},
                SquareState::WhiteBishop => html! {<SafeHtml html={include_str!("./pieces/BW.svg")} />},
                SquareState::WhiteKnight => html! {<SafeHtml html={include_str!("./pieces/KnW.svg")} />},
                SquareState::WhitePawn => html! {<SafeHtml html={include_str!("./pieces/PW.svg")} />},
                SquareState::BlackKing => html! {<SafeHtml html={include_str!("./pieces/KB.svg")} />},
                SquareState::BlackQueen => html! {<SafeHtml html={include_str!("./pieces/QB.svg")} />},
                SquareState::BlackRook => html! {<SafeHtml html={include_str!("./pieces/RB.svg")} />},
                SquareState::BlackBishop => html! {<SafeHtml html={include_str!("./pieces/BB.svg")} />},
                SquareState::BlackKnight => html! {<SafeHtml html={include_str!("./pieces/KnB.svg")} />},
                SquareState::BlackPawn => html! {<SafeHtml html={include_str!("./pieces/PB.svg")} />},
                _ => html! {},
            }}
        </div>
        }
    }

#[function_component(BoardSquare)]
fn board_square(square: &SquareProps) -> Html {
    html! {
        <div class={format!("square  {:?}", square.square.color )} >
            { square.square.name.clone() }
            { if square.state != SquareState::Empty {
                    html!{<BoardPiece state={square.state.clone()}/>}
                } else {
                    html!{}
                }
            }
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let squares = ChessBoard::get_base_board();
    let states: Vec<SquareState> = ChessBoard::get_initial_position();

    html! {
        <main class="main">
            <div style="width: 810px; flex-direction: row; flex-wrap: wrap; display: flex;">
                { 
                    for squares.iter().map(|square| html!{<BoardSquare square ={ square.clone()} state={states[rank_and_file_to_index(square.rank, square.file)].clone()}/>})
                }
            </div>
        </main>
    }
}
