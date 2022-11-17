export class Student {
  uid: number;
  firstname: String;
  lastname: String;
  birthday: Date;
  school_class: String;
  printed_in: Date;
  valid_to: Date;
  image: String;

  constructor(
    uid: number,
    firstname: String,
    lastname: String,
    birth_date: Date,
    school_class: String,
    printed_in: Date,
    valid_to: Date,
    image: String
  ){
    this.uid = uid;
    this.firstname = firstname;
    this.lastname = lastname;
    this.birthday = birth_date;
    this.school_class = school_class;
    this.printed_in = printed_in;
    this.valid_to = valid_to;
    this.image = image;
  }
}
