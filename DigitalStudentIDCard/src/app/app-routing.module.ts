import { StudentCardComponent } from './student-card/student-card.component';
import { LoginComponent } from './login/login.component';
import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { VerificationComponent } from './verification/verification.component';
import { CanActivateRoute } from './router-guard';

const routes: Routes = [
  {
    path: '',
    redirectTo: '/login',
    pathMatch: 'full'
  },
  {
    path: 'login',
    component: LoginComponent

  },
  {
    path: 'student',
    component: StudentCardComponent,
    canActivate: [CanActivateRoute]
  },
  {
    path: 'verification',
    component: VerificationComponent
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
