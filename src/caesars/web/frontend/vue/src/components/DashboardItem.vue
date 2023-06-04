<script setup lang="ts">
import { ref } from 'vue'
import { gql, useClientHandle } from '@urql/vue'

const client = useClientHandle().client

const plain = ref('')
const secret = ref('')

const plainPlaceholder = ref('me@caesar.tld')
const secretPlaceholder = ref('zr@pnfne.gyq')

function encrypt() {
  const encryptQuery = gql`
    query ($plain: String!) {
      encrypt(plain: $plain, rotation: 13) {
        secret
      }
    }
  `

  client
    .query(encryptQuery, { plain: plain.value })
    .toPromise()
    .then((response) => {
      secret.value = response.data.encrypt.secret
    })
    .catch((err) => console.log(err))
}
function decrypt() {
  const encryptQuery = gql`
    query ($secret: String!) {
      decrypt(secret: $secret, rotation: 13) {
        plain
      }
    }
  `

  client
    .query(encryptQuery, { secret: secret.value })
    .toPromise()
    .then((response) => {
      plain.value = response.data.decrypt.plain
    })
    .catch((err) => console.log(err))
}
</script>

<template>
  <section class="mt-10 flex flex-col">
    <div class="mb-6 rounded bg-gray-200 pt-3">
      <label for="plain">Plain</label>
      <textarea
        id="plain"
        :placeholder="plainPlaceholder"
        v-model="plain"
        @input="encrypt"
      ></textarea>
    </div>

    <div class="mb-6 rounded bg-gray-200 pt-3">
      <label for="secret">Secret</label>
      <textarea
        id="secret"
        :placeholder="secretPlaceholder"
        v-model="secret"
        @input="decrypt"
      ></textarea>
    </div>

    <div class="flex justify-center"></div>
  </section>
</template>

<style scoped>
textarea {
  @apply w-full border-b-4 border-b-4 border-gray-300 border-gray-300 bg-gray-200 px-3 pb-3 text-gray-700 transition duration-500;
  &:focus {
    @apply border-yellow-600 outline-none;
  }
}

label {
  @apply mb-2 ml-3 block text-sm font-bold text-gray-700;
}
</style>
