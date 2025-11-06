fn main() {
   let name = "Fahd Animashaun";
   let uni:&str = "PAU";
   let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
   println!("University{}, \nAddress: {}",uni,addr );

   let department:&'static str = "Computer Science";
   let school:&'static str = "School of science and Technology";
   println!("Department: {}, \nSchool: {}",department,school );
}
