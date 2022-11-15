import { Component, OnInit, ViewEncapsulation } from '@angular/core';
import {
  trigger,
  state,
  style,
  transition,
  animate,
} from '@angular/animations';
import { StudentService } from './student.service';
import { Student } from './student';
import { Router } from '@angular/router';

@Component({
  selector: 'app-student-card',
  encapsulation: ViewEncapsulation.None,
  templateUrl: './student-card.component.html',
  styleUrls: ['./student-card.component.css'],
  animations: [
    trigger('flipState', [
      state(
        'active',
        style({
          transform: 'rotateY(179deg)',
        })
      ),
      state(
        'inactive',
        style({
          transform: 'rotateY(0)',
        })
      ),
      transition('active => inactive', animate('500ms ease-out')),
      transition('inactive => active', animate('500ms ease-in')),
    ]),
  ],
})
export class StudentCardComponent implements OnInit {
  model: Student;
  qrData = '';

  constructor(private studentService: StudentService, private router: Router) {
    this.model = studentService.getStudentData();
    this.qrData = this.router.url + '/verification?token=' + this.studentService.getToken();
  }

  ngOnInit(): void {}

  flip: string = 'inactive';

  toggleFlip() {
    this.flip = this.flip == 'inactive' ? 'active' : 'inactive';
  }
}
