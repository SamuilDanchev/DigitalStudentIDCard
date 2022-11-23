import { Injectable } from "@angular/core";
import { CanActivate, Router } from "@angular/router";
import { User } from '../app/login/user';
import { LoginComponent } from "./login/login.component";

@Injectable({
  providedIn: 'root'
})
export class CanActivateRoute implements CanActivate {
    
    constructor(private login: LoginComponent, private router: Router) {}

    canActivate() {
      if (!this.login.login() === true) {
        return true;
      } else {
        window.alert("Keine Zugangsberechtigung");
        this.router.navigateByUrl('/login');
        return false;
      }
    }
  }