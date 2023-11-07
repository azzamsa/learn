<script lang="ts">
  /* global EncryptResponse, DecryptResponse */
  import { gql, getContextClient } from "@urql/svelte"
  let client = getContextClient()

  let plain = ""
  let secret = ""

  let plainPlaceholder = "me@caesar.tld"
  let secretPlaceholder = "zr@pnfne.gyq"

  function encrypt() {
    const encryptQuery = gql`
      query ($plain: String!) {
        encrypt(plain: $plain, rotation: 13) {
          secret
        }
      }
    `

    client
      .query(encryptQuery, { plain: plain })
      .toPromise()
      .then((response: EncryptResponse) => {
        if (response.data) {
          secret = response.data.encrypt.secret
        }
      })
      .catch((err) => console.log(err))
  }
  function decrypt() {
    const decryptQuery = gql`
      query ($secret: String!) {
        decrypt(secret: $secret, rotation: 13) {
          plain
        }
      }
    `

    client
      .query(decryptQuery, { secret: secret })
      .toPromise()
      .then((response: DecryptResponse) => {
        if (response.data) {
          plain = response.data.decrypt.plain
        }
      })
      .catch((err) => console.log(err))
  }
</script>

<section class="mt-10 flex flex-col">
  <div class="mb-6 rounded bg-gray-200 pt-3">
    <label class="input-label" for="plain">Plain</label>
    <textarea id="plain" placeholder={plainPlaceholder} bind:value={plain} on:input={encrypt} />
  </div>

  <div class="mb-6 rounded bg-gray-200 pt-3">
    <label class="input-label" for="secret">Secret</label>
    <textarea id="secret" placeholder={secretPlaceholder} bind:value={secret} on:input={decrypt} />
  </div>

  <div class="flex justify-center" />
</section>

<style lang="postcss">
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
