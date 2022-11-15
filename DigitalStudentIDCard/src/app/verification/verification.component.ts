import { Component, OnInit } from '@angular/core';
import { VerifyService } from '../student-card/verify.service';

@Component({
  selector: 'app-verification',
  templateUrl: './verification.component.html',
  styleUrls: ['./verification.component.css']
})
export class VerificationComponent implements OnInit {

  msg = 'Der Schüler '

  student = ''

  constructor(private verifyService: VerifyService) {
    this.verifyService.verify().subscribe(res => {
      console.log()
    })
  }

  ngOnInit(): void {
  }

}
