import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { User } from '../login/user';
import { Student } from './student';
import { GlobVar } from '../globals';
import { LoginRes } from '../login/logingRes';
import { Router } from '@angular/router';

@Injectable({ providedIn: 'root' })
export class StudentService {

  private userData: Student = {
    uid: 1,
    firstname: 'John',
    lastname: 'Doe',
    birth_date: new Date('2001-01-01'),
    school_class: 'WIT3C',
    printed_in: new Date('2010-01-01'),
    valid_to: new Date('2012-01-01'),
  } ;
   private token = 'token123';

  constructor(private http: HttpClient, private router: Router) {}

  login(user: User) {
    this.http.post<LoginRes>(GlobVar.LOGIN_URL, user).subscribe(res => {
      this.userData = res.student;
      this.token = res.token;

    });
  }

  getStudentData() {
    return this.userData;
  }

  getToken() {
    return this.token;
  }
}
