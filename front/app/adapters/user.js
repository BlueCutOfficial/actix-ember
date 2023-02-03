import RESTAdapter from '@ember-data/adapter/rest';

export default class UserAdapter extends RESTAdapter {
  host = 'http://127.0.0.1:8080';
  namespace = 'api';
}
