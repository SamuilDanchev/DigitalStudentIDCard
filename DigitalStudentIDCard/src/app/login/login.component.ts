import { Router } from '@angular/router';
import { HttpClient } from '@angular/common/http';
import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { FormGroup, FormControl } from '@angular/forms';
import { Student } from '../student-card/student';
import { StudentService } from '../student-card/student.service';
import { User } from './user';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css'],
})
export class LoginComponent implements OnInit {
  @Input() error: string | undefined;

  @Output() submitEM = new EventEmitter();

  constructor(private router: Router, private http: HttpClient, private studentService: StudentService) {}

  ngOnInit(): void {}

  form: FormGroup = new FormGroup({
    username: new FormControl(''),
    password: new FormControl(''),
  });

  submit() {
    if (this.form.valid) {
      this.submitEM.emit(this.form.value);
      this.router.navigate(['/student']);
    }
  }

  login() {
    const user: User =
      {
        username: this.form.get('username')?.value,
        password: this.form.get('password')?.value,
      }

    this.studentService.login(user);
  }
}
