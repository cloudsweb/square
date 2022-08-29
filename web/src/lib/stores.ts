import { defineStore, acceptHMRUpdate } from 'pinia'

export class UserLoginInfo {
  alias?: string
  password?: string
}

export class UserCreateInfo {
  alias?: string
  name?: string
  password?: string
  email?: string
  desc?: string
}

function parse_jwt(token: string) {
  const base64 = token.split('.')[1].replace(/-/g, '+').replace(/_/g, '/');
  const jsonPayload = decodeURIComponent(atob(base64).split('').map(function(c) {
      return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
  }).join(''));

  return JSON.parse(jsonPayload)
}

async function login(alias: string, password: string) {
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
      const token = result.token

      const tokenContent = parse_jwt(token);
      const sub = tokenContent.sub as string
      let id = 0
      if (typeof sub == 'string' && sub.startsWith('#')) {
        id = parseInt(sub.slice(1))
      }

      console.log(`token: ${result.token}`)

      // await this.get_user_info()
      return { alias, token, id }
    }
  } catch (e) {
    console.warn("login failed", e)
  }
}

async function signup(info: UserCreateInfo) {
  try {
    await fetch('/api/users/create', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(info),
    })
    return true
  } catch (e) {
    console.warn("signup failed", e)
  }
  return false
}

export const useUserStore = defineStore({
  id: 'user',
  state: () => ({
    _id: 0,
    _alias: null as string | null,
    _name: '',
  }),
  getters: {
    id: (state) => state._id,
    alias: (state) => state._alias,
    name: (state) => state._name,
  },
  actions: {
    async login(login_info: UserLoginInfo) {
      if (login_info.alias == null || login_info.password == null) {
        console.warn("alias or password should not be null");
        return;
      }
      const info = await login(login_info.alias, login_info.password)
      if (info == null) {
        console.warn("login failed")
        return
      }
      this._id = info.id
      this._alias = info.alias
      this._name = "TODO: my test"
    },

    async signup(login_info: UserCreateInfo) {
      return await signup(login_info)
    },

    async logout() {
      this._id = 0
      this._alias = null
    }
  }
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useUserStore, import.meta.hot))
}
