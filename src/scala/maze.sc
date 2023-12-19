package maze

sealed trait Exploration
case object UnExplored extends Exploration
case object LeftExplored extends Exploration
case object Explored extends Exploration

sealed trait Maze
case class Branch(label: String, left: Maze, right: Maze) extends Maze {
  var status: Exploration = UnExplored
}
case class Leaf(label: String) extends Maze

object Maze {
  def explore(maze: Maze, stack: List[Maze], labels: List[String]): (List[Maze], List[String]) = maze match {
    case branch@Branch(label: String, left: Maze, right: Maze) =>
      branch.status match { // maze.status won't work, status is not a member of maze: Maze
        case UnExplored => {
          branch.status = LeftExplored;
          explore(left, maze::stack, label::labels)
        }
        case LeftExplored => {
          branch.status = Explored;
          explore(right, stack, label::labels)
        }; 
        case Explored =>
          (stack, label::labels)
      }
    case Leaf(label) =>
      (stack, label::labels)
  }
  def label(maze:Maze) = maze match {
    case Branch(label, _, _) => label
    case Leaf(label) => label
  }
}

object Test extends App {
  import Maze.explore

  val leaf2 = Leaf("2")
  val leaf4 = Leaf("4")
  val leaf5 = Leaf("5")
  val leaf8 = Leaf("8")
  val branch3 = Branch("3", leaf4, leaf5)
  val branch1 = Branch("1", leaf2, branch3)
  val branch7 = Branch("7", leaf5, leaf8)
  val branch6 = Branch("6", branch3, branch7)
  val branch0 = Branch("0", branch1, branch6)

  def loop(pair: (List[Maze], List[String])): (List[Maze], List[String]) = pair._1 match {
    case Nil => pair
    case maze::stack => {
      println(s"exploring node ${Maze.label(maze)}")
      loop(explore(maze, stack, pair._2))
    }
  }
  val (_, labels) = loop(List(branch0), List())
  println(s"explored nodes: ${labels.reverse}")
}