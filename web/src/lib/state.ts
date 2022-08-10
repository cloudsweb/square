export default class State {
  alias?: string
  nickname?: string
  id?: string
  token?: string

  parse_jwt(token: string) {
    const base64 = token.split('.')[1].replace(/-/g, '+').replace(/_/g, '/');
    const jsonPayload = decodeURIComponent(atob(base64).split('').map(function(c) {
        return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
    }).join(''));

    return JSON.parse(jsonPayload)
  }

  async login(alias: string, password: string) {
    try {
      const resp = await fetch('/api/users/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ alias, password }),
      })

      const result = await resp.json()

      // TODO: check user.alias
      if (result.token != null && result.token != '') {
        this.alias = alias
        this.token = result.token

        const tokenContent = this.parse_jwt(this.token!);
        let id = tokenContent.sub as string
        if (id.startsWith('#')) {
          id = id.substr(1)
        }
        this.id = id

        console.log(`token: ${result.token}`)

        await this.get_user_info()
      }
    } catch (e) {
      console.warn("login failed", e)
    }
  }

  async get_user_info() {
    try {
      const resp = await fetch(`/api/users/${this.id}/info`, {
        headers: {
          'Authorization': 'Bearer ' + this.token
        }
      })
      const result = await resp.json()
      this.nickname = result.nickname
    } catch (e) {
      console.warn("get_user_info failed", e)
    }
  }
}
