import { Injectable } from "@angular/core";
import { CanActivate, Router } from "@angular/router";
import { LoginComponent } from "./login/login.component";
import { User } from "./login/user";
import { StudentService } from "./student-card/student.service";

@Injectable({
  providedIn: 'root'
})
export class CanActivateRoute implements CanActivate {
    
    constructor(private login: StudentService, private router: Router) {}

    canActivate() {
      if (this.login.result === true) {
        return true;
      } else {
        window.alert("Keine Zugangsberechtigung");
        this.router.navigateByUrl('/login');
        return false;
      }
    }
  }