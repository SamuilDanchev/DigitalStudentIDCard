import { Student } from "../student-card/student";

export interface LoginRes {
  token: string;
  student: Student;
}
