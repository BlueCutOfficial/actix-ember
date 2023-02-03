import Model, { attr } from '@ember-data/model';

export default class UserModel extends Model {
  @attr('string') login;
  @attr('string') name;
}
