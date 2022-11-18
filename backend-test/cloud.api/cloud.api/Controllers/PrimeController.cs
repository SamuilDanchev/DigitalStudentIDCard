using Microsoft.AspNetCore.Mvc;
using System.Collections;
using System.Text.Json;

namespace cloud.api.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class LoginController : ControllerBase
    {
        private readonly ILogger<LoginController> _logger;

        public LoginController(ILogger<LoginController> logger)
        {
            _logger = logger;
        }

        [HttpPost()]
        public IActionResult Get()
        {
            var student = new StudentModel
            {
                token = "ein Token",
                student =
                {
                    uid = 420,
                    firstname = "Lars",
                    lastname = "Born",
                    birthday = "1977-01-07",
                    school_class = "Eine Klasse",
                    printed_in = "2004-02-08",
                    valid_to = "2005-09-10",
                    image = "https://i.natgeofe.com/n/548467d8-c5f1-4551-9f58-6817a8d2c45e/NationalGeographic_2572187_square.jpg",
                }
            };

            var opt = new JsonSerializerOptions() { WriteIndented = true };
            string strStudent = JsonSerializer.Serialize<StudentModel>(student, opt);


            return Ok(strStudent);
        }
    }
}