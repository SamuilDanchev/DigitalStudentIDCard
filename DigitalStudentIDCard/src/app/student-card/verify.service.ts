import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { GlobVar } from '../globals';

@Injectable({ providedIn: 'root' })
export class VerifyService {

  private token = ''

  constructor(private router: ActivatedRoute, private http: HttpClient) {}

  verify() {
    this.router.queryParams
      .subscribe(params => {
        this.token = params['token'];
      })

    return this.http.post(GlobVar.VERIFY_URL + '/authorization?token=' + this.token , {})
  }
}
