import { Injectable } from "@angular/core";
import { CanActivate } from "@angular/router";
import { User } from '../app/login/user';
import { LoginComponent } from "./login/login.component";

@Injectable()
export class CanActivateRoute implements CanActivate {

    user!: User;

    constructor(private login: LoginComponent) {}

    canActivate() {
      if (this.login.login() === true) {
        return true;
      } else {
        window.alert("Keine Zugangsberechtigung");
        return false;
      }
    }
  }