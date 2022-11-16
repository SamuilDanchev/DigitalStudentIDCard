export class Student {
  uid: number;
  firstname: String;
  lastname: String;
  birth_date: Date;
  school_class: String;
  printed_in: Date;
  valid_to: Date;

  constructor(
    uid: number,
    firstname: String,
    lastname: String,
    birth_date: Date,
    school_class: String,
    printed_in: Date,
    valid_to: Date
  ){
    this.uid = uid;
    this.firstname = firstname;
    this.lastname = lastname;
    this.birth_date = birth_date;
    this.school_class = school_class;
    this.printed_in = printed_in;
    this.valid_to = valid_to;
  }
}
