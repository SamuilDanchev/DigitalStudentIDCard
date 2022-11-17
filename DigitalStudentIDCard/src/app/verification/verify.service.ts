import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { GlobVar } from '../globals';
import { VerifyRes } from './verifyRes';

@Injectable({ providedIn: 'root' })
export class VerifyService {

  private token = ''

  constructor(private router: ActivatedRoute, private http: HttpClient) {}

  verify() {
    this.router.queryParams
      .subscribe(params => {
        this.token = params['token'];
      })

      const headers = new HttpHeaders({
        'Content-Type': 'application/json',
        Authorization: `${this.token}`,
      });

    return this.http.get<VerifyRes>(GlobVar.VERIFY_URL , {headers:  headers})
  }
}
