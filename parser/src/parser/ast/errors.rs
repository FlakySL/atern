use thiserror::Error;

#[derive(Error, Debug)]
<<<<<<< HEAD
=======
<<<<<<< HEAD
pub enum AstErr {}
=======
>>>>>>> a22607f (chore!: rowan removed from the project. Parser implementation needed.)
pub enum AstErr {
    #[error("Terminal node: {0} cannot have children")]
    TerminalNodeChildAdition(String),
}
<<<<<<< HEAD
=======
>>>>>>> 6613d25 (chore!: rowan removed from the project. Parser implementation needed.)
>>>>>>> a22607f (chore!: rowan removed from the project. Parser implementation needed.)
