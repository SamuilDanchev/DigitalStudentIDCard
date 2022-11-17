import { Component, OnInit } from '@angular/core';
import { VerifyService } from './verify.service';

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
      this.msg += res.firstname + ' ' + res.lastname + ' ist verifiziert!'
    }, error => {
      this.msg += ' ist nicht verifiziert!'
    }

    )
  }

  ngOnInit(): void {
  }

}
