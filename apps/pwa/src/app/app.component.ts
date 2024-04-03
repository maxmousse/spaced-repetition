import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { RouterModule } from '@angular/router';
import { Apollo, gql } from 'apollo-angular';
import { map } from 'rxjs';

const GET_USERS = gql`
  query GetUsers {
    getUsers {
      id
      firstName
      lastName
      email
    }
  }
`;

@Component({
  standalone: true,
  imports: [RouterModule, CommonModule],
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent {
  title = 'pwa';
  users$ = this.graphql
    .query({ query: GET_USERS })
    .pipe(map((result) => result.data));

  constructor(private graphql: Apollo) {}
}
