//json-server --watch db.json
import axios from 'axios'

const apiClient = axios.create({
   baseURL: 'http://localhost:3000',
   withCredentials: false,
   headers: {
       Accept: 'application/json',
       'Content-Type': 'application/json'
   } 
})

export default {
    getTodos() {
        return apiClient.get('/todos',{withCredentials: true})
    },
    putTodo(todo, id) {
        return apiClient.put('/events', todo)
    }
}