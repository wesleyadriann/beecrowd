use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let notes: Vec<&str> = input.split_whitespace().collect();

  let n1 = notes[0].parse::<f64>().unwrap();
  let n2 = notes[1].parse::<f64>().unwrap();
  let n3 = notes[2].parse::<f64>().unwrap();
  let n4 = notes[3].parse::<f64>().unwrap();

  let median = ((n1 * 2.0) + (n2 * 3.0) + (n3 * 4.0) + (n4 * 1.0)) / 10.0;

  if median > 6.9 {
    println!("Media: {:.1}
Aluno aprovado.", median);
    return;
  } else if median >= 5.0 && median <= 6.9 {
    let mut input_exam = String::new();
    io::stdin().read_line(&mut input_exam).unwrap();

    let exam = input_exam.trim().parse::<f64>().unwrap();

    let exam_median = (median + exam) / 2.0;
    if exam_median >= 5.0 {
      println!("Media: {:.1}
Aluno em exame.
Nota do exame: {:.1}
Aluno aprovado.
Media final: {:.1}", median, exam, exam_median);
      return ;
    }
  }

    println!("Media: {:.1}
Aluno reprovado.", median);
}